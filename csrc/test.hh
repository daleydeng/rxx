#pragma once

#include <cstdint>
#include <vector>

std::vector<int64_t> dummy_cpp_new_vector_i64(int a);

void dummy_cpp_add_vector_i64(std::vector<int64_t>& val, int n);
int64_t dummy_cpp_addret_vector_i64(std::vector<int64_t>& v, int n);
int64_t dummy_cpp_get_vector_i64(std::vector<int64_t>const& v);
void dummy_cpp_getvoid_vector_i64(std::vector<int64_t>const& v, int a);
