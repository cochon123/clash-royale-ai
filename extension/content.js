const CAPTURE_TIMEOUT_MS = 15000;
const MAX_REPLAY_ATTEMPTS = 5;

let capturedCount = 0;
let captureSequence = 0;
let latestCapture = null;
const captureWaiters = new Set();

function sendMessage(message) {
  return new Promise((resolve) => {
    try {
      if (!chrome.runtime?.id) {
        resolve(null);
        return;
      }
      chrome.runtime.sendMessage(message, (result) => {
        // Reading lastError prevents Chrome from reporting a rejected callback.
        if (chrome.runtime.lastError) {
          resolve(null);
          return;
        }
        resolve(result ?? null);
      });
    } catch {
      // An extension reload invalidates old content-script contexts until the tab reloads.
      resolve(null);
    }
  });
}

function classifyPayload(payload) {
  if (
    payload?.success === true &&
    typeof payload?.html === "string"
  ) {
    return { ok: true, rateLimited: false, retryAfter: 0 };
  }
  const rateLimited = payload?.status === 429 || payload?.error_code === 1015;
  return {
    ok: false,
    rateLimited,
    retryAfter: Number(payload?.retry_after) || 30
  };
}

window.addEventListener("message", (event) => {
  if (
    event.source !== window ||
    event.origin !== location.origin ||
    event.data?.source !== "cr-replay-collector" ||
    event.data?.type !== "replay-payload"
  ) {
    return;
  }

  captureSequence += 1;
  const capture = {
    sequence: captureSequence,
    ...classifyPayload(event.data.payload)
  };
  latestCapture = capture;
  for (const notify of captureWaiters) notify();
  captureWaiters.clear();

  sendMessage({
      type: "captured-replay",
      requestUrl: event.data.requestUrl,
      referrerUrl: event.data.referrerUrl,
      payload: event.data.payload
    }).then((result) => {
      if (capture.ok) {
        capturedCount += 1;
        const storage = result?.classification || result?.storage || "captured";
        renderStatus(`Valid replays: ${capturedCount} (${storage})`);
      }
    });
});

function panel() {
  let root = document.getElementById("cr-replay-panel");
  if (root) return root;
  root = document.createElement("section");
  root.id = "cr-replay-panel";
  root.innerHTML = `
    <strong>CR Replay Collector</strong>
    <span id="cr-replay-status">Ready</span>
    <button id="cr-replay-action" type="button">Collect</button>
  `;
  document.documentElement.appendChild(root);
  return root;
}

function renderStatus(text) {
  panel().querySelector("#cr-replay-status").textContent = text;
}

function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForRateLease() {
  const lease = (await sendMessage({ type: "rate-lease" })) || { wait_ms: 5000 };
  const waitMs = Math.max(0, Number(lease.wait_ms) || 0);
  if (waitMs > 0) {
    renderStatus(`Global pacing: ${(waitMs / 1000).toFixed(1)}s`);
    await wait(waitMs);
  }
}

async function waitForCapture(afterSequence, timeoutMs = CAPTURE_TIMEOUT_MS) {
  if (latestCapture?.sequence > afterSequence) return latestCapture;
  return new Promise((resolve) => {
    let settled = false;
    const finish = () => {
      if (settled || latestCapture?.sequence <= afterSequence) return;
      settled = true;
      clearTimeout(timer);
      captureWaiters.delete(finish);
      resolve(latestCapture);
    };
    const timer = setTimeout(() => {
      if (settled) return;
      settled = true;
      captureWaiters.delete(finish);
      resolve(null);
    }, timeoutMs);
    captureWaiters.add(finish);
  });
}

async function loadAllReplayButtons() {
  let stable = 0;
  let previous = -1;
  let previousHeight = -1;
  while (stable < 2) {
    const count = document.querySelectorAll("button.replay_button").length;
    renderStatus(`Loading replay list: ${count}`);
    const height = document.body.scrollHeight;
    stable = count === previous && height === previousHeight ? stable + 1 : 0;
    previous = count;
    previousHeight = height;
    window.scrollTo(0, height);
    await wait(800);
  }
  return [...document.querySelectorAll("button.replay_button")];
}

function isHeroCard(element) {
  const evidence = [
    element.getAttribute("class"),
    element.getAttribute("src"),
    element.getAttribute("data-card-type"),
    element.getAttribute("data-rarity"),
    element.getAttribute("data-hero")
  ].filter(Boolean).join(" ").toLowerCase();
  const level = Number(
    element.getAttribute("data-evolution-level") ||
    element.getAttribute("data-evolution") || 0
  );
  return level >= 2 || /(^|[\/_ -])hero([\/_ .-]|$)/.test(evidence);
}

function heroPrefilter(button) {
  const battle = button.closest(
    ".battle, .battle_container, .battle_item, [data-battle-id]"
  );
  if (!battle) return { eligible: true, confident: false };

  const selectors = [
    ".battle_team", ".battle_opponent", ".team", ".opponent",
    "[data-side='team']", "[data-side='opponent']",
    "[data-team='true']", "[data-opponent='true']"
  ];
  const sides = [...new Set(selectors.flatMap(
    (selector) => [...battle.querySelectorAll(selector)]
  ))].filter((side) => side.querySelectorAll("img").length >= 4);

  // Reject only when both deck containers are recognizable. Unknown layouts
  // remain eligible so a site redesign cannot silently discard good data.
  if (sides.length !== 2) return { eligible: true, confident: false };
  const heroCounts = sides.map((side) =>
    [...side.querySelectorAll("img, [data-card-type], [data-hero]")]
      .filter(isHeroCard).length
  );
  return {
    eligible: heroCounts.every((count) => count <= 1),
    confident: true,
    heroCounts
  };
}

async function collectReplay(button, index, total) {
  for (let attempt = 1; attempt <= MAX_REPLAY_ATTEMPTS; attempt += 1) {
    renderStatus(`Replay ${index + 1}/${total}; attempt ${attempt}`);
    await waitForRateLease();
    button.scrollIntoView({ block: "center" });
    const before = captureSequence;
    button.click();
    const capture = await waitForCapture(before);

    if (capture?.ok) {
      return true;
    }

    if (capture?.rateLimited) {
      const exponential = 30 * 2 ** (attempt - 1);
      const seconds = Math.min(
        180,
        Math.max(capture.retryAfter, exponential)
      );
      renderStatus(`Rate limited; retrying in ${seconds}s`);
      await wait(seconds * 1000 + Math.floor(Math.random() * 1500));
      continue;
    }

    const seconds = Math.min(30, 3 * 2 ** (attempt - 1));
    renderStatus(`No replay received; retrying in ${seconds}s`);
    await wait(seconds * 1000);
  }
  return false;
}

async function collectBattlePage() {
  const ranked = document.querySelector(
    '.battle_filter .item[data-type="ranked1v1"]'
  );
  if (ranked && !ranked.classList.contains("active")) {
    ranked.click();
    await wait(1800);
  }

  const allButtons = await loadAllReplayButtons();
  const decisions = allButtons.map((button) => ({ button, ...heroPrefilter(button) }));
  const buttons = decisions.filter((decision) => decision.eligible).map((decision) => decision.button);
  const skipped = decisions.length - buttons.length;
  const inspected = decisions.filter((decision) => decision.confident).length;
  if (allButtons.length === 0) {
    renderStatus("No replay buttons found; player deferred");
    if (new URL(location.href).searchParams.get("crcollector") === "1") {
      sendMessage({
        type: "collection-done",
        retryPlayer: true,
        error: "no replay buttons found"
      });
    }
    return;
  }
  if (buttons.length === 0) {
    renderStatus(`Done: all ${skipped} multi-Hero battles skipped`);
    if (new URL(location.href).searchParams.get("crcollector") === "1") {
      sendMessage({ type: "collection-done", retryPlayer: false, error: null });
    }
    return;
  }
  let missed = 0;
  for (let index = 0; index < buttons.length; index += 1) {
    if (!(await collectReplay(buttons[index], index, buttons.length))) {
      missed += 1;
    }
  }

  renderStatus(
    `Done: ${capturedCount} valid; ${skipped} multi-Hero skipped; ` +
    `${inspected}/${allButtons.length} decks inspected; ${missed} deferred`
  );
  if (new URL(location.href).searchParams.get("crcollector") === "1") {
    sendMessage({
      type: "collection-done",
      retryPlayer: missed > 0,
      error: missed > 0 ? `${missed} replay requests failed` : null
    });
  }
}

function collectLeaderboard() {
  const paths = [
    ...new Set(
      [...document.querySelectorAll('a[href^="/player/"]')]
        .map((link) => link.getAttribute("href"))
        .filter((path) => /^\/player\/[A-Z0-9]+\/?$/.test(path || ""))
    )
  ];
  renderStatus(`Queueing ${paths.length} player pages`);
  sendMessage({ type: "open-player-pages", paths }).then((result) => {
    renderStatus(
      `Added: ${result?.added ?? 0}; queued: ${result?.queued ?? 0}; active: ${result?.active ?? 0}`
    );
  });
}

document.addEventListener("DOMContentLoaded", () => {
  const challenge = /just a moment|attention required|checking your browser/i.test(
    `${document.title} ${document.body?.innerText?.slice(0, 500) || ""}`
  );
  if (challenge) {
    sendMessage({
      type: "collector-problem",
      reason: `Cloudflare challenge at ${location.href}`
    });
    return;
  }

  sendMessage({ type: "browser-ready" });
  const root = panel();
  const action = root.querySelector("#cr-replay-action");
  if (location.pathname.includes("/players/leaderboard")) {
    action.textContent = "Queue visible players";
    action.addEventListener("click", collectLeaderboard);
  } else if (location.pathname.includes("/battles")) {
    action.textContent = "Collect ranked replays";
    action.addEventListener("click", collectBattlePage);
    if (new URL(location.href).searchParams.get("crcollector") === "1") {
      setTimeout(collectBattlePage, 1500);
    }
  } else {
    action.disabled = true;
    renderStatus("Open a leaderboard or battles page");
  }
});
