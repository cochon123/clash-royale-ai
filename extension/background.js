const API_BASE = "http://127.0.0.1:8765";
const FALLBACK_WORKER_COUNT = 2;
const MAX_WORKER_COUNT = 8;
const WORKER_STALL_MS = 4 * 60 * 1000;
const workers = new Map(); // tab id -> {id, tag}
const finishingTabs = new Set();
let recentLoginRedirects = [];
let recentChallengeEvents = [];
const challengeSeenAt = new Map();
let pumping = false;
let initialization = null;

async function api(path, body = null) {
  const options = body === null
    ? {}
    : {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body)
      };
  const response = await fetch(`${API_BASE}${path}`, options);
  if (!response.ok) throw new Error(`${path}_${response.status}`);
  return response.json();
}

function playerUrl(tag, workerId) {
  return `https://royaleapi.com/player/${encodeURIComponent(tag)}/battles?crcollector=1&crworker=${workerId}`;
}

async function recoverWorkers(reloadTabs = false) {
  const tabs = await chrome.tabs.query({});
  const recovered = [];
  for (const tab of tabs) {
    try {
      const url = new URL(tab.url);
      const workerId = url.searchParams.get("crworker");
      const match = url.pathname.match(/^\/player\/([^/]+)\/battles$/);
      if (url.origin === "https://royaleapi.com" && workerId && match) {
        const tag = decodeURIComponent(match[1]);
        if (!workers.has(tab.id)) {
          const duplicate = [...workers.values()].some((worker) => worker.id === workerId);
          if (duplicate) {
            try {
              await api("/jobs/complete", {
                tag,
                retry: true,
                error: "duplicate worker tab recovered after extension reload"
              });
              await chrome.tabs.remove(tab.id).catch(() => {});
            } catch {
              // Keep the tab and lease intact if the daemon cannot accept recovery.
            }
            continue;
          }
          workers.set(tab.id, { id: workerId, tag, lastActivityAt: Date.now() });
        }
        recovered.push(tab.id);
      }
    } catch {
      // Chrome internal tabs do not have parseable URLs.
    }
  }
  if (reloadTabs) {
    await Promise.all(recovered.map((tabId) => chrome.tabs.reload(tabId).catch(() => {})));
  }
}

function freeWorkerId() {
  const used = new Set([...workers.values()].map((worker) => worker.id));
  for (let index = 1; index <= MAX_WORKER_COUNT; index += 1) {
    const id = `chrome-${index}`;
    if (!used.has(id)) return id;
  }
  return null;
}

async function desiredWorkerCount() {
  try {
    const status = await api("/status");
    if (status.collector?.paused) return 0;
    const recommended = Number(status.collector?.rate?.recommended_workers);
    if (Number.isFinite(recommended)) {
      return Math.max(0, Math.min(MAX_WORKER_COUNT, recommended));
    }
  } catch {
    // Retain the current safe experiment size if status is temporarily unavailable.
  }
  return FALLBACK_WORKER_COUNT;
}

async function pump() {
  if (pumping) return;
  pumping = true;
  try {
    const target = await desiredWorkerCount();
    while (workers.size < target) {
      const workerId = freeWorkerId();
      if (!workerId) break;
      const claimed = await api("/jobs/claim", { worker_id: workerId });
      if (!claimed.job) break;
      const tab = await chrome.tabs.create({
        url: playerUrl(claimed.job.tag, workerId),
        active: false
      });
      workers.set(tab.id, {
        id: workerId,
        tag: claimed.job.tag,
        lastActivityAt: Date.now()
      });
    }
  } catch {
    // The daemon may be offline; the alarm or next RoyaleAPI page will retry.
  } finally {
    pumping = false;
  }
}

async function finishWorker(tabId, retry, error) {
  if (finishingTabs.has(tabId)) return;
  const worker = workers.get(tabId);
  if (!worker) return;
  finishingTabs.add(tabId);
  try {
    await api("/jobs/complete", { tag: worker.tag, retry, error });
    const target = await desiredWorkerCount();
    if (workers.size > target) {
      workers.delete(tabId);
      await chrome.tabs.remove(tabId).catch(() => {});
      await pump();
      return;
    }
    const claimed = await api("/jobs/claim", { worker_id: worker.id });
    if (claimed.job) {
      worker.tag = claimed.job.tag;
      worker.lastActivityAt = Date.now();
      await chrome.tabs.update(tabId, { url: playerUrl(worker.tag, worker.id), active: false });
    } else {
      workers.delete(tabId);
      await chrome.tabs.remove(tabId).catch(() => {});
    }
    await pump();
  } finally {
    finishingTabs.delete(tabId);
  }
}

function touchWorker(tabId) {
  const worker = workers.get(tabId);
  if (worker) worker.lastActivityAt = Date.now();
}

async function recoverStalledWorkers() {
  let status;
  try {
    status = await api("/status");
  } catch {
    return;
  }
  if (status.collector?.paused) return;

  const now = Date.now();
  const leases = new Map(
    (status.collector?.active || []).map((lease) => [
      `${lease.worker_id}:${lease.tag}`,
      Number(lease.lease_until) * 1000
    ])
  );
  for (const [tabId, worker] of [...workers.entries()]) {
    const leaseUntil = leases.get(`${worker.id}:${worker.tag}`) || 0;
    const silentFor = now - (worker.lastActivityAt || now);
    if (leaseUntil < now || silentFor >= WORKER_STALL_MS) {
      await finishWorker(
        tabId,
        true,
        leaseUntil < now ? "expired worker lease" : "worker silent for four minutes"
      ).catch(() => {});
    }
  }
}

async function handleCloudflareChallenge(tabId, reason) {
  const worker = workers.get(tabId);
  if (!worker) return { ok: true, ignored: "not a worker" };

  const now = Date.now();
  const key = String(tabId);
  if (now - (challengeSeenAt.get(key) || 0) < 5000) {
    return { ok: true, ignored: "duplicate challenge signal" };
  }
  challengeSeenAt.set(key, now);
  recentChallengeEvents = recentChallengeEvents.filter(
    (timestamp) => now - timestamp < 2 * 60 * 1000
  );
  recentChallengeEvents.push(now);

  if (recentChallengeEvents.length >= 3) {
    recentChallengeEvents = [];
    await api("/problem", {
      reason: "Three Cloudflare challenges occurred within two minutes"
    });
    await finishWorker(tabId, true, reason);
    return { ok: true, paused: true };
  }

  await finishWorker(tabId, true, reason);
  return { ok: true, recycled: true };
}

async function clearLegacySingleChallengePause() {
  try {
    const status = await api("/status");
    const reason = status.collector?.pause_reason || "";
    if (
      status.collector?.paused &&
      /^Cloudflare challenge in worker tab \d+$/.test(reason)
    ) {
      await api("/control", { action: "resume" });
    }
  } catch {
    // The normal alarm retry will recover when the daemon returns.
  }
}

async function fallbackDownload(record) {
  const tag = new URL(record.request_url).searchParams.get("tag") || Date.now();
  const url = `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(record))}`;
  await chrome.downloads.download({
    url,
    filename: `cr-replays/${tag}.json`,
    conflictAction: "uniquify",
    saveAs: false
  });
}

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.type === "captured-replay") {
    if (sender.tab) touchWorker(sender.tab.id);
    const record = {
      schema_version: 1,
      captured_at: new Date().toISOString(),
      request_url: message.requestUrl,
      referrer_url: message.referrerUrl,
      payload: message.payload
    };
    api("/replays", record)
      .then((result) => sendResponse({ ok: true, storage: "local-server", ...result }))
      .catch(async () => {
        await fallbackDownload(record);
        sendResponse({ ok: true, storage: "download" });
      });
    return true;
  }

  if (message.type === "rate-lease") {
    if (sender.tab) touchWorker(sender.tab.id);
    const workerId = sender.tab ? workers.get(sender.tab.id)?.id : "browser";
    api("/rate/lease", { worker_id: workerId || "browser" })
      .then(sendResponse)
      .catch((error) => sendResponse({ ok: false, error: String(error), wait_ms: 5000 }));
    return true;
  }

  if (message.type === "open-player-pages") {
    const tags = message.paths
      .map((path) => path.match(/^\/player\/([A-Z0-9]+)\/?$/)?.[1])
      .filter(Boolean);
    api("/players/seed", { tags, source: "leaderboard" })
      .then(async (result) => {
        await pump();
        sendResponse({ ok: true, ...result, active: workers.size });
      })
      .catch((error) => sendResponse({ ok: false, error: String(error) }));
    return true;
  }

  if (message.type === "collection-done" && sender.tab) {
    finishWorker(sender.tab.id, Boolean(message.retryPlayer), message.error)
      .then(() => sendResponse({ ok: true }))
      .catch((error) => sendResponse({ ok: false, error: String(error) }));
    return true;
  }

  if (message.type === "collector-problem") {
    const reason = message.reason || "browser requires attention";
    const action =
      sender.tab && reason.startsWith("Cloudflare challenge")
        ? handleCloudflareChallenge(sender.tab.id, reason)
        : api("/problem", { reason });
    action
      .then(sendResponse)
      .catch((error) => sendResponse({ ok: false, error: String(error) }));
    return true;
  }

  if (message.type === "browser-ready") {
    if (sender.tab) touchWorker(sender.tab.id);
    pump().then(() => sendResponse({ ok: true, active: workers.size }));
    return true;
  }
});

chrome.tabs.onRemoved.addListener((tabId) => {
  const worker = workers.get(tabId);
  if (!worker) return;
  workers.delete(tabId);
  api("/jobs/complete", {
    tag: worker.tag,
    retry: true,
    error: "worker tab closed"
  }).finally(pump);
});

chrome.tabs.onUpdated.addListener((tabId, changeInfo) => {
  if (!workers.has(tabId) || !changeInfo.url) return;
  touchWorker(tabId);
  if (changeInfo.url.includes("__cf_chl")) {
    handleCloudflareChallenge(
      tabId,
      `Cloudflare challenge in worker tab ${tabId}`
    ).catch(() => {});
    return;
  }
  if (new URL(changeInfo.url).pathname.startsWith("/login")) {
    const now = Date.now();
    recentLoginRedirects = recentLoginRedirects.filter((timestamp) => now - timestamp < 60000);
    recentLoginRedirects.push(now);
    if (recentLoginRedirects.length >= 2) {
      api("/problem", { reason: "Repeated RoyaleAPI login redirects across workers" })
        .then(() => finishWorker(tabId, true, "RoyaleAPI login required"))
        .catch(() => {});
    } else {
      finishWorker(tabId, true, "single RoyaleAPI login redirect").catch(() => {});
    }
  }
});

function start(reloadWorkers = false) {
  if (!initialization) {
    initialization = (async () => {
      await recoverWorkers(false);
      await chrome.alarms.create("collector-pump", { periodInMinutes: 0.5 });
      await clearLegacySingleChallengePause();
      await recoverStalledWorkers();
      await pump();
    })();
  }
  if (reloadWorkers) {
    return initialization.then(async () => {
      await recoverWorkers(true);
      await pump();
    });
  }
  return initialization;
}

chrome.alarms.onAlarm.addListener((alarm) => {
  if (alarm.name === "collector-pump") {
    recoverStalledWorkers().then(pump);
  }
});
chrome.runtime.onStartup.addListener(() => start(false));
chrome.runtime.onInstalled.addListener(() => start(true));
start(false);
