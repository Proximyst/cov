const { min, max } = require("./helpers");

function add(a, b) {
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
module.exports = { add };
