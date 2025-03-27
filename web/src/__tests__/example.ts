import { expect, test } from "@jest/globals";

test.concurrent("example", async () => {
  expect(1 + 1).toBe(2);
});
