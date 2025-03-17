#include "./sample.h"
#include "./helpers.h"

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
