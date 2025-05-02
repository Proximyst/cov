#include "./sample.h"
#include <stdio.h>

int main() {
  for (int i = 0; i < 500; ++i) {
    int32_t result = add(2, 3);
    if (result != 5) {
      printf("expected 2+3 to be 5. got: %d\n", result);
      return 1;
    }
  }

  int32_t result = add(9, 10);
  if (result != 21) {
    printf("expected 9+10 to be 21. got: %d\n", result);
    return 1;
  }

  return 0;
}
