"use strict";
const test = require("node:test");
const assert = require("node:assert/strict");
const ArenaInputTransport = require("./static/input-transport.js");

const flush = () => new Promise((resolve) => setImmediate(resolve));
const command = (op, serial, extra = {}) => ({ op, serial, match_id: 1,
  slot: 0, card_id: 26, x: 4500, y: 13000, ...extra });
function connection(onError) {
  const requests = [];
  let active = 0, maxActive = 0;
  const transport = new ArenaInputTransport({ onError, send(value, signal) {
    active++;
    maxActive = Math.max(maxActive, active);
    return new Promise((resolve, reject) => requests.push({ value, signal,
      finish(error) { active--; error ? reject(error) : resolve({ accepted: true }); } }));
  } });
  transport.reset(1);
  return { transport, requests, maxActive: () => maxActive };
}

test("hundreds of mouse moves retain only the latest unsent aim", async () => {
  const { transport, requests, maxActive } = connection();
  transport.enqueue(command("select", 1));
  for (let serial = 2; serial <= 501; serial++) transport.enqueue(command("aim", serial, { x: serial }));
  assert.equal(requests.length, 1);
  assert.equal(transport.stats.queued, 1);
  assert.equal(transport.stats.pendingAims, 1);
  assert.equal(transport.stats.coalesced, 499);
  requests[0].finish();
  await flush();
  assert.equal(requests.length, 2);
  assert.equal(requests[1].value.serial, 501);
  assert.equal(requests[1].value.x, 501);
  requests[1].finish();
  await flush();
  assert.equal(transport.stats.queued, 0);
  assert.equal(transport.stats.inFlight, 0);
  assert.equal(maxActive(), 1);
});

test("play follows only the in-flight request, never a pointer history or obsolete select", async () => {
  const { transport, requests } = connection();
  transport.enqueue(command("select", 1));
  for (let serial = 2; serial <= 400; serial++) transport.enqueue(command("aim", serial));
  transport.enqueue(command("select", 401, { slot: 1, card_id: 27 }));
  transport.enqueue(command("aim", 402, { slot: 1, card_id: 27 }));
  transport.enqueue(command("play", 403, { slot: 1, card_id: 27 }));
  assert.equal(transport.stats.queued, 1);
  assert.equal(transport.stats.pendingAims, 0);
  requests[0].finish();
  await flush();
  assert.deepEqual(requests.map((request) => request.value.op), ["select", "play"]);
  assert.equal(requests[1].value.serial, 403);
  requests[1].finish();
  await flush();
});

test("accepted plays stay ordered while newer play supersedes an intervening select", async () => {
  const { transport, requests, maxActive } = connection();
  for (const [op, serial] of [["select", 1], ["play", 2], ["select", 3], ["play", 4]]) {
    assert.equal(transport.enqueue(command(op, serial)), true);
  }
  assert.equal(transport.enqueue(command("select", 3)), false);
  requests[0].finish(); await flush();
  requests[1].finish(); await flush();
  requests[2].finish(); await flush();
  assert.deepEqual(requests.map((request) => request.value.serial), [1, 2, 4]);
  assert.equal(maxActive(), 1);
});

test("cancel clears pending preview intent without discarding an accepted play", async () => {
  const { transport, requests } = connection();
  transport.enqueue(command("select", 1));
  transport.enqueue(command("play", 2));
  transport.enqueue(command("select", 3));
  transport.enqueue(command("aim", 4));
  transport.enqueue(command("cancel", 5));
  assert.equal(transport.stats.queued, 2);
  requests[0].finish(); await flush();
  requests[1].finish(); await flush();
  requests[2].finish(); await flush();
  assert.deepEqual(requests.map((request) => request.value.op), ["select", "play", "cancel"]);
});

test("reset aborts old fetch, drops old queue and ignores its late error", async () => {
  const errors = [];
  const { transport, requests, maxActive } = connection((error) => errors.push(error));
  transport.enqueue(command("select", 200));
  transport.enqueue(command("play", 201));
  transport.reset(2);
  assert.equal(requests[0].signal.aborted, true);
  assert.equal(transport.enqueue(command("play", 1, { match_id: 2 })), true);
  assert.equal(transport.enqueue(command("play", 9999)), false);
  assert.equal(requests.length, 1); // old adapter may ignore AbortSignal
  requests[0].finish(new Error("old round's delayed failure"));
  await flush();
  assert.deepEqual(errors, []);
  assert.equal(transport.stats.failed, 0);
  assert.equal(requests[1].value.match_id, 2);
  assert.equal(requests[1].value.serial, 1);
  requests[1].finish(); await flush();
  assert.equal(maxActive(), 1);
});

test("destroy prevents queued sends and late error callbacks", async () => {
  const errors = [];
  const { transport, requests } = connection((error) => errors.push(error));
  transport.enqueue(command("select", 1));
  transport.enqueue(command("play", 2));
  transport.destroy();
  transport.destroy();
  assert.equal(requests[0].signal.aborted, true);
  assert.equal(transport.enqueue(command("play", 3)), false);
  requests[0].finish(new Error("closed")); await flush();
  assert.equal(requests.length, 1);
  assert.equal(transport.stats.queued, 0);
  assert.equal(transport.stats.destroyed, true);
  assert.deepEqual(errors, []);
});

test("a failed request and a throwing UI callback cannot wedge the queue", async () => {
  let errors = 0;
  const { transport, requests } = connection(() => { errors++; throw new Error("UI error"); });
  transport.enqueue(command("select", 1));
  transport.enqueue(command("play", 2));
  requests[0].finish(new Error("network lost")); await flush();
  assert.equal(errors, 1);
  assert.equal(requests[1].value.op, "play");
  requests[1].finish(); await flush();
  assert.equal(transport.stats.failed, 1);
  assert.equal(transport.stats.completed, 1);
});

test("essential commands have a hard bound and overflow is reported, never silently discarded", async () => {
  const errors = [];
  const { transport, requests } = connection((error) => errors.push(error));
  transport.enqueue(command("select", 1));
  for (let serial = 2; serial <= 1001; serial++) {
    assert.equal(transport.enqueue(command("play", serial)), serial <= 17);
  }
  assert.equal(transport.stats.queued, 16);
  assert.equal(transport.stats.maxQueued, 16);
  assert.equal(errors.length, 984);
  assert.equal(errors[0].code, "ARENA_INPUT_BACKLOG");
  for (let i = 0; i <= 16; i++) { requests[i].finish(); await flush(); }
  assert.deepEqual(requests.slice(1).map((request) => request.value.serial),
    Array.from({ length: 16 }, (_, index) => index + 2));
});

test("commands are copied; bad serials or wrong matches never poison the barrier", async () => {
  const { transport, requests } = connection();
  assert.equal(transport.enqueue(command("play", 1000, { match_id: 9 })), false);
  assert.equal(transport.enqueue(command("play", NaN)), false);
  assert.equal(transport.enqueue(command("play", 0.5)), false);
  const value = command("play", 1);
  assert.equal(transport.enqueue(value), true);
  value.x = 9999;
  assert.equal(requests[0].value.x, 4500);
  assert.equal(Object.isFrozen(requests[0].value), true);
  requests[0].finish(); await flush();
});

test("a synchronous send failure also drains subsequent commands", async () => {
  const errors = [], sent = [];
  const transport = new ArenaInputTransport({ send(value) {
    sent.push(value.serial);
    if (value.serial === 1) throw new Error("request could not start");
  }, onError: (error) => errors.push(error) });
  transport.enqueue(command("select", 1));
  transport.enqueue(command("play", 2));
  await flush();
  assert.deepEqual(sent, [1, 2]);
  assert.equal(errors.length, 1);
  assert.equal(transport.stats.inFlight, 0);
});
