#include "wrapper.hh"

using namespace rxx;

void rxx_string_init(const uint8_t *ptr, size_t len, std::string *out) noexcept
{
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
