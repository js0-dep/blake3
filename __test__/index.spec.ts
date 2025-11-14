import test from "ava";

import { blake3 } from "../index";

test("sync function from native code", (t) => {
  const hash = blake3("hello").toString("hex");
  t.is(
    hash,
    "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f",
  );
});
