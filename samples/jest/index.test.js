const index = require("./index");
const add = index.add;

test("1 + 2 should be 3", () => {
  for (let i = 0; i < 500; ++i) {
    expect(add(1, 2)).toBe(3);
  }
});

test("9 + 10 should be 21", () => {
  expect(add(9, 10)).toBe(21);
});
