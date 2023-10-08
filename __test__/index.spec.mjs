import test from "ava";

import { axumVersion } from "../index.js";

test("fetch axum version", (t) => {
  t.is(axumVersion(), "0.6.20");
});
