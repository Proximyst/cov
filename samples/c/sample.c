#include <stdint.h>
#include "./sample.h"

static int32_t min(int32_t a, int32_t b) {
    return a < b ? a : b;
}

static int32_t max(int32_t a, int32_t b) {
    return a > b ? a : b;
}

int32_t add(int32_t a, int32_t b) {
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
