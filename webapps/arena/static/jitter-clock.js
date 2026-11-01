/* Adaptive display lag for the tick stream, and the scene clock the fake painter steps. */
(function (root) {
  "use strict";
  const TICKS_PER_SECOND = 20;
  const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));

  class JitterClock {
    constructor() {
      this.arrivals = [];
      this.jitterMs = 0;
      this.lagMs = 150;
      this.lastArrival = null;
    }
    noteArrival(wallMs) {
      this.lastArrival = wallMs;
      this.arrivals.push(wallMs);
      const cutoff = wallMs - 10000;
      while (this.arrivals.length && this.arrivals[0] < cutoff) this.arrivals.shift();
      const gaps = [];
      for (let i = 1; i < this.arrivals.length; i++) gaps.push(this.arrivals[i] - this.arrivals[i - 1]);
      if (gaps.length) {
        const sorted = gaps.slice().sort((a, b) => a - b);
        const index = Math.min(sorted.length - 1, Math.max(0, Math.ceil(sorted.length * 0.95) - 1));
        this.jitterMs = sorted[index];
        this.lagMs = clamp(this.jitterMs + 50, 100, 300);
      }
      return this.lagMs;
    }
    // The lag stays while the stream is quiet. The scene clock stops one tick
    // short of the newest snapshot on its own, and slows down to rebuild the
    // lag when ticks resume. Collapsing the lag to zero here made the clock
    // run up to the newest tick and then freeze until the lag was rebuilt.
    appliedLagMs() {
      return this.lagMs;
    }
    lagTicks(nowMs, speed = 1) {
      const rate = TICKS_PER_SECOND * clamp(speed, 0, 16);
      return this.appliedLagMs(nowMs) / 1000 * rate;
    }
  }

  // Mirrors SceneDisplayClock in engine/native-spectator/src/clock.rs for the
  // fake painter. The rate follows the distance to the lag target, it never
  // passes one tick before the newest snapshot, and it never moves back.
  class SceneDisplayClock {
    constructor() {
      this.display = null;
      this.lag = 3;
      this.catchingUp = false;
    }
    setLag(ticks) { this.lag = Number.isFinite(ticks) ? Math.max(0, ticks) : 0; }
    reset() { this.display = null; this.catchingUp = false; }
    target(oldest, latest, flow) {
      if (flow === "finished") return latest;
      return Math.max(oldest, latest - this.lag);
    }
    limit(oldest, latest, flow) {
      return flow === "running" ? Math.max(oldest, latest - Math.min(this.lag, 1)) : this.target(oldest, latest, flow);
    }
    caughtUp(oldest, latest, flow) {
      return this.display != null && this.display + 1e-6 >= this.limit(oldest, latest, flow);
    }
    step(oldest, latest, speed, flow, dt) {
      const target = this.target(oldest, latest, flow);
      // A burst observed before the first paint starts at the lagged edge.
      // Walking in from the oldest tick would leave that burst on screen forever.
      let display;
      if (this.display == null) display = latest - oldest <= this.lag ? oldest : target;
      else if (this.display >= oldest - 0.5 && this.display <= latest + 0.5) display = this.display;
      else display = oldest;
      const rate = clamp(speed, 0, 16);
      const advance = Math.min(0.25, Math.max(0, dt)) * TICKS_PER_SECOND * rate;
      if (flow === "running") {
        const behind = target - display;
        if (behind < 1) this.catchingUp = false;
        let factor = 1;
        if (behind > 100) display = target;
        else if (this.catchingUp || behind > 4) { this.catchingUp = true; factor = 1 + Math.min(behind / (20 * Math.max(rate, 1)), 1); }
        else if (behind > 1) factor = 1 + (behind - 1) / 30;
        else if (behind < 0) factor = Math.max(1 + behind / 10, 0.75);
        const limit = this.limit(oldest, latest, flow);
        if (display < limit) display = Math.min(display + advance * factor, limit);
      } else if (flow === "finished") display = Math.min(display + advance, target);
      else if (flow === "paused" && display < target) display = Math.min(display + advance, target);
      display = Math.min(latest, Math.max(oldest, display));
      this.display = display;
      return display;
    }
  }

  // The scene clock no longer moves back when the lag grows, so the desired
  // lag is passed through. Kept for callers of the old contract.
  function lagTicksForDisplay(desiredTicks) {
    return Number.isFinite(desiredTicks) ? Math.max(0, desiredTicks) : 0;
  }

  root.JitterClock = JitterClock;
  root.SceneDisplayClock = SceneDisplayClock;
  root.lagTicksForDisplay = lagTicksForDisplay;
  if (typeof module !== "undefined" && module.exports) module.exports = {JitterClock, SceneDisplayClock, lagTicksForDisplay, TICKS_PER_SECOND};
})(typeof globalThis !== "undefined" ? globalThis : this);
