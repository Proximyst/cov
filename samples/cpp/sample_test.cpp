#include "./sample.hpp"
#include <iostream>

int main() {
  sample::Adder adder;
  for (int i = 0; i < 500; ++i) {
    int32_t result = adder.add(2, 3);
    if (result != 5) {
      std::cout << "expected 2+3 to be 5. got: " << result << std::endl;
      return 1;
    }
  }

  int32_t result = adder.add(9, 10);
  if (result != 21) {
    std::cout << "expected 9+10 to be 21. got: " << result << std::endl;
    return 1;
  }

  return 0;
}
