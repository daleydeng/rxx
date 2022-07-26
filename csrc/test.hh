#pragma once

#include <cstdint>
#include <vector>
#include <memory>

std::vector<int64_t> dummy_cpp_new_vector_i64(int a);

void dummy_cpp_add_vector_i64(std::vector<int64_t>& val, int n);
int64_t dummy_cpp_addret_vector_i64(std::vector<int64_t>& v, int n);
int64_t dummy_cpp_get_vector_i64(std::vector<int64_t>const& v);
void dummy_cpp_getvoid_vector_i64(std::vector<int64_t>const& v, int a);

int64_t const & dummy_cpp_getref_vector_i64(std::vector<int64_t> const &v, int idx);

struct Dummy {
  int64_t *data_;
  size_t len_;

  Dummy(int64_t *data, size_t len): data_(data), len_(len) {}

  int64_t get(size_t idx) const {return data_[idx];}

  int64_t &get_mut(size_t idx) {return data_[idx];}

  void add(int64_t val) {
    for (size_t i = 0; i < len_; i++)
      data_[i] += val;
  }

  std::unique_ptr<Dummy> create(int64_t *data, size_t len) {
    return std::make_unique<Dummy>(data, len);
  }
};
