#ifndef SAMPLE_HPP
#define SAMPLE_HPP
#include <cstdint>

namespace sample {
class Adder {
public:
  // Adds two numbers together.
  // This also applies some business logic for some combinations of numbers.
  int32_t add(int32_t a, int32_t b);
};
} // namespace sample
#endif // SAMPLE_HPP
