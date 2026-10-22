"use strict";

// A pointer can produce many more previews than a connection can acknowledge.
// Keep only its latest intent, while delivering card plays in serial order.
(function (root) {
  const PREVIEW_OPS = new Set(["aim", "select", "cancel"]);
  const MAX_ESSENTIAL = 16;

  class ArenaInputTransport {
    constructor({ send, onError = () => {} }) {
      if (typeof send !== "function") throw new TypeError("send must be a function");
      this._send = send;
      this._onError = onError;
      this._queue = [];
      this._flight = null;
      this._generation = 0;
      this._matchId = undefined;
      this._serial = -1;
      this._destroyed = false;
      this._counts = { sent: 0, completed: 0, failed: 0, coalesced: 0, dropped: 0, maxQueued: 0 };
    }

    get stats() {
      return { ...this._counts, queued: this._queue.length,
        pendingAims: Number(this._queue.some((command) => command.op === "aim")),
        inFlight: Number(this._flight !== null), matchId: this._matchId,
        destroyed: this._destroyed, maxEssential: MAX_ESSENTIAL };
    }

    reset(matchId) {
      if (this._destroyed) return;
      this._generation++;
      this._counts.dropped += this._queue.length;
      this._queue = [];
      this._matchId = matchId;
      this._serial = -1;
      // send(command, signal) may pass this signal to fetch. Still retain the
      // flight barrier until it settles, even if a send ignores cancellation.
      this._flight?.abort.abort();
    }

    destroy() {
      if (this._destroyed) return;
      this.reset(undefined);
      this._destroyed = true;
    }

    enqueue(command) {
      if (this._destroyed || !command || typeof command.op !== "string"
          || !Number.isSafeInteger(command.serial) || command.serial <= 0 || command.serial <= this._serial
          || command.match_id === undefined
          || (this._matchId !== undefined && command.match_id !== this._matchId)) {
        this._counts.dropped++;
        return false;
      }
      // Allow the first frame's command to establish the round, although the
      // caller should reset(roundId) whenever the authoritative round changes.
      if (this._matchId === undefined) this._matchId = command.match_id;

      let next = this._queue;
      if (command.op === "aim") {
        next = next.filter((pending) => pending.op !== "aim");
      } else if (PREVIEW_OPS.has(command.op) || command.op === "play") {
        // Play is self-contained: a queued select/aim/cancel is unnecessary.
        // A newer selection/cancel also supersedes all older preview intent.
        // Previously accepted plays remain ordered and are never discarded.
        next = next.filter((pending) => !PREVIEW_OPS.has(pending.op));
      }
      if (command.op !== "aim"
          && next.filter((pending) => pending.op !== "aim").length >= MAX_ESSENTIAL) {
        this._counts.dropped++;
        const error = new Error("Input connection is busy. Wait for your pending card plays.");
        error.code = "ARENA_INPUT_BACKLOG";
        this._error(error);
        return false;
      }
      this._counts.coalesced += this._queue.length - next.length;
      this._queue = next;
      this._queue.push(Object.freeze({ ...command }));
      this._serial = command.serial;
      this._counts.maxQueued = Math.max(this._counts.maxQueued, this._queue.length);
      this._pump();
      return true;
    }

    _error(error) {
      // A UI callback cannot break the transport's drain loop.
      try { this._onError(error); } catch { /* UI owns its error reporting. */ }
    }

    async _pump() {
      if (this._destroyed || this._flight || !this._queue.length) return;
      const command = this._queue.shift();
      const flight = { generation: this._generation, abort: new AbortController() };
      this._flight = flight;
      this._counts.sent++;
      try {
        await this._send(command, flight.abort.signal);
        if (!this._destroyed && flight.generation === this._generation) this._counts.completed++;
      } catch (error) {
        if (!this._destroyed && flight.generation === this._generation) {
          this._counts.failed++;
          this._error(error);
        }
      } finally {
        this._flight = null;
        this._pump();
      }
    }
  }

  root.ArenaInputTransport = ArenaInputTransport;
  if (typeof module !== "undefined" && module.exports) module.exports = ArenaInputTransport;
})(typeof window === "undefined" ? globalThis : window);
