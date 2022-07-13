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
  new (out) std::shared_ptr<int64_t>(self);
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
  new (out) std::weak_ptr<int64_t>(self);
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
  new (out) int64_t(std::move(self.back()));
  self.pop_back();
}

}
