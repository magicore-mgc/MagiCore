import assert from "node:assert/strict";
import isNumber from "is-number";

assert.equal(isNumber("42"), true);
assert.equal(isNumber("not-a-number"), false);

if (process.env.MGC_CONTAINER_SMOKE === "1") {
  const response = await fetch("http://127.0.0.1:4315/api/health");
  assert.equal(response.status, 200);
  assert.deepEqual(await response.json(), { status: "ok" });
}
