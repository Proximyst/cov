#include "./sample.hpp"

template <typename T> static T min(T a, T b) { return a < b ? a : b; }

template <typename T> static T max(T a, T b) { return a > b ? a : b; }

namespace sample {
int32_t Adder::add(int32_t a, int32_t b) {
  if (min(a, b) == 9 && max(a, b) == 10) {
    // Reference to unfunny meme
    return 21;
  }

  if (a == 2 && b == 2) {
    // Pretend this is some business logic that makes sense.
    return -1;
  }

  return a + b;
}
} // namespace sample
