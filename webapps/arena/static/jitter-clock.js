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
    // A quiet stream collapses the lag so the clock can reach the newest tick and hold.
    appliedLagMs(nowMs) {
      if (this.lastArrival == null) return this.lagMs;
      if (nowMs - this.lastArrival <= this.lagMs) return this.lagMs;
      return 0;
    }
    lagTicks(nowMs, speed = 1) {
      const rate = TICKS_PER_SECOND * clamp(speed, 0, 16);
      return this.appliedLagMs(nowMs) / 1000 * rate;
    }
  }

  class SceneDisplayClock {
    constructor() {
      this.display = null;
      this.lag = 3;
    }
    setLag(ticks) { this.lag = Number.isFinite(ticks) ? Math.max(0, ticks) : 0; }
    reset() { this.display = null; }
    target(oldest, latest, flow) {
      if (flow === "finished") return latest;
      return Math.max(oldest, latest - this.lag);
    }
    caughtUp(oldest, latest, flow) {
      return this.display != null && this.display + 1e-6 >= this.target(oldest, latest, flow);
    }
    step(oldest, latest, speed, flow, dt) {
      const target = this.target(oldest, latest, flow);
      // A burst observed before the first paint starts at the lagged edge.
      // Walking in from the oldest tick would leave that burst on screen forever.
      let display;
      if (this.display == null) display = latest - oldest <= this.lag ? oldest : target;
      else if (this.display >= oldest - 0.5 && this.display <= latest + 0.5) display = this.display;
      else display = oldest;
      const advance = Math.min(0.25, Math.max(0, dt)) * TICKS_PER_SECOND * clamp(speed, 0, 16);
      if (flow === "running" || flow === "finished") display = Math.min(display + advance, target);
      else if (flow === "paused" && display < target) display = Math.min(display + advance, target);
      display = Math.min(latest, Math.max(oldest, display));
      this.display = display;
      return display;
    }
  }

  // Restoring lag after a stall must not rewind the tick already on screen.
  function lagTicksForDisplay(desiredTicks, display, latest) {
    const desired = Number.isFinite(desiredTicks) ? Math.max(0, desiredTicks) : 0;
    if (display == null || latest == null || !Number.isFinite(display) || !Number.isFinite(latest)) return desired;
    return Math.min(desired, Math.max(0, latest - display));
  }

  root.JitterClock = JitterClock;
  root.SceneDisplayClock = SceneDisplayClock;
  root.lagTicksForDisplay = lagTicksForDisplay;
  if (typeof module !== "undefined" && module.exports) module.exports = {JitterClock, SceneDisplayClock, lagTicksForDisplay, TICKS_PER_SECOND};
})(typeof globalThis !== "undefined" ? globalThis : this);
