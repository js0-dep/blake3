import test from "ava";

import { blake } from "../index";

test("sync function from native code", (t) => {
  const hash = blake("hello");
  console.log(hash);
  // const fixture = 42
  // t.is(plus100(fixture), fixture + 100)
});
