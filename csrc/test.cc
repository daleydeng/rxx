#include "wrapper.hh"
#include "test.hh"

using namespace rxx;

std::vector<int64_t> dummy_cpp_new_vector_i64(int v) {
  return {v};
}

void dummy_cpp_add_vector_i64(std::vector<int64_t>& v, int n) {
  v[0] += n;
}

extern "C" {

void rxx_dummy_new_unique_i64(int64_t v, std::unique_ptr<int64_t> *out) {
  new (out) std::unique_ptr<int64_t>(new int64_t(v));
}

void rxx_dummy_new_shared_i64(int64_t v, std::shared_ptr<int64_t> *out) {
  new (out) std::shared_ptr<int64_t>(new int64_t(v));
}

void rxx_dummy_new_vector_i64(const int64_t *data, size_t len, std::vector<int64_t> *out) {
  new (out) std::vector<int64_t>;
  out->assign(data, data+len);
}

void rxx_dummy_new_unique_string(std::unique_ptr<std::string> *out) {
  new (out) std::unique_ptr<std::string>(new std::string("test"));
}

void rxx_dummy_new_shared_string(std::shared_ptr<std::string> *out) {
  new (out) std::shared_ptr<std::string>(new std::string("test"));
}

} // extern "C"
