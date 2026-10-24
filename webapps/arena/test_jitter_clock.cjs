/* Run with: node --test webapps/arena/test_jitter_clock.cjs */
const test = require("node:test");
const assert = require("node:assert/strict");
const {JitterClock, SceneDisplayClock, lagTicksForDisplay} = require("./static/jitter-clock.js");

test("display lag is the p95 inter-arrival plus 50 ms, clamped to 100–300", () => {
  const clock = new JitterClock();
  let wall = 0;
  for (let i = 0; i < 40; i++) {
    wall += i < 30 ? 40 : 180;
    clock.noteArrival(wall);
  }
  assert.ok(clock.jitterMs >= 180);
  assert.equal(clock.lagMs, 230);
  const tight = new JitterClock();
  for (let i = 0; i < 20; i++) tight.noteArrival(i * 10);
  assert.equal(tight.jitterMs, 10);
  assert.equal(tight.lagMs, 100);
  const wide = new JitterClock();
  for (let i = 0; i < 20; i++) wide.noteArrival(i * 400);
  assert.equal(wide.lagMs, 300);
});

test("arrivals older than 10 s leave the p95 window", () => {
  const clock = new JitterClock();
  clock.noteArrival(0);
  clock.noteArrival(500);
  assert.equal(clock.jitterMs, 500);
  clock.noteArrival(11000);
  clock.noteArrival(11020);
  assert.equal(clock.jitterMs, 20);
  assert.equal(clock.lagMs, 100);
});

test("a stall collapses lag to zero and a live stream keeps it", () => {
  const clock = new JitterClock();
  for (let i = 0; i < 8; i++) clock.noteArrival(i * 50);
  assert.equal(clock.appliedLagMs(7 * 50 + 40), clock.lagMs);
  assert.equal(clock.appliedLagMs(7 * 50 + clock.lagMs + 1), 0);
  assert.equal(clock.lagTicks(7 * 50 + 10, 1), clock.lagMs / 1000 * 20);
  assert.equal(clock.lagTicks(7 * 50 + 1000, 1), 0);
  assert.equal(clock.lagTicks(7 * 50 + 10, 4), clock.lagMs / 1000 * 80);
});

test("the scene clock advances toward the lagged target and holds when caught up", () => {
  const clock = new SceneDisplayClock();
  clock.setLag(3);
  assert.equal(clock.step(0, 2, 1, "running", 1 / 30), 0);
  let shown = clock.step(0, 10, 1, "running", 1 / 30);
  assert.ok(shown > 0);
  assert.ok(shown < 7);
  for (let i = 0; i < 90; i++) shown = clock.step(0, 10, 1, "running", 1 / 30);
  assert.ok(Math.abs(shown - 7) < 1e-6);
  assert.equal(clock.caughtUp(0, 10, "running"), true);
  const held = clock.step(0, 10, 1, "running", 1 / 30);
  assert.equal(held, shown);
});

test("a burst wider than the lag starts at the lagged live edge instead of the oldest tick", () => {
  const clock = new SceneDisplayClock();
  clock.setLag(3);
  const shown = clock.step(0, 40, 1, "running", 1 / 30);
  assert.ok(Math.abs(shown - 37) < 1e-6);
});

test("clearing lag lets a stalled clock run down to the newest tick and then hold", () => {
  const clock = new SceneDisplayClock();
  clock.setLag(3);
  clock.step(0, 10, 1, "running", 1 / 30);
  clock.setLag(0);
  let shown = clock.display;
  for (let i = 0; i < 40; i++) shown = clock.step(0, 10, 1, "running", 1 / 30);
  assert.equal(shown, 10);
  assert.equal(clock.step(0, 10, 1, "running", 1 / 30), 10);
});

test("restoring lag after a stall does not rewind the displayed tick", () => {
  assert.equal(lagTicksForDisplay(8, 10, 10), 0);
  assert.equal(lagTicksForDisplay(8, 10, 12), 2);
  assert.equal(lagTicksForDisplay(3, null, 12), 3);
  const clock = new SceneDisplayClock();
  clock.setLag(0);
  for (let i = 0; i < 40; i++) clock.step(0, 10, 1, "running", 1 / 30);
  assert.equal(clock.display, 10);
  clock.setLag(lagTicksForDisplay(3, clock.display, 10));
  assert.equal(clock.step(0, 10, 1, "running", 1 / 30), 10);
});

test("ticks waiting past the lag window move the clock instead of repeating the old tick", () => {
  const clock = new SceneDisplayClock();
  clock.setLag(1);
  const first = clock.step(0, 0, 1, "running", 1 / 30);
  assert.equal(first, 0);
  const next = clock.step(0, 5, 1, "running", 1 / 30);
  assert.ok(next > first);
  assert.ok(next <= 4);
});
