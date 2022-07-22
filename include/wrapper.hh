#pragma once

#include <memory>
#include <iostream>
#include <vector>

namespace rxx {

template <typename T>
void destroy(T *ptr) {
  ptr->~T();
}

template<typename T>
void shared_ptr_clone(const std::shared_ptr<T> &self, std::shared_ptr<T> *out)
{
  new (out) std::shared_ptr<T>(self);
}

template<typename T>
void weak_ptr_upgrade(const std::weak_ptr<T> &self, std::shared_ptr<T> *out)
{
  new (out) std::shared_ptr<T>(self.lock());
}

template<typename T>
void weak_ptr_downgrade(const std::shared_ptr<T> &self, std::weak_ptr<T> *out)
{
  new (out) std::weak_ptr<T>(self);
}

template<typename T>
void weak_ptr_clone(const std::weak_ptr<T> &self, std::weak_ptr<T> *out)
{
  new (out) std::weak_ptr<T>(self);
}

template<typename T>
size_t vector_size(const std::vector<T> &self)
{
  return self.size();
}

template<typename T>
const T& vector_get(const std::vector<T> &self, size_t pos) {
  return self[pos];
}

template<typename T>
T& vector_get_mut(std::vector<T> &self, size_t pos) {
  return self[pos];
}

template<typename T>
void vector_push_back(std::vector<T> &self, T &value) {
  self.push_back(std::move(value));
  destroy(&value);
}

template<typename T>
void vector_pop_back(std::vector<T> &self, T *out) {
  new (out) T(std::move(self.back()));
  self.pop_back();
}

} // namespace rxx

extern "C" {
  void rxx_string_init(const uint8_t *ptr, size_t len, std::string *out) noexcept;
  void rxx_string_destroy(std::string *self) noexcept;
  size_t rxx_string_length(const std::string &self) noexcept;
  const char* rxx_string_data(const std::string &self) noexcept;
  void rxx_string_clear(std::string &self) noexcept;
  void rxx_string_reserve(std::string &self, size_t n) noexcept;
  void rxx_string_push(std::string &self, const uint8_t *ptr, size_t len) noexcept;
}
