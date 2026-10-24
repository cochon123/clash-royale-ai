"use strict";
// Original sprites are cached and composited locally. Authoritative combat stays on the server.
// The explicit stream view remains available for comparison and compatibility.
const $ = (id) => document.getElementById(id);
const BASE = new URL(".", document.currentScript.src).pathname.replace(/\/$/, "");
const matchId = new URLSearchParams(location.search).get("id");
const canvas = $("board"), fallbackCanvas = $("board-fallback"), ctx = fallbackCanvas.getContext("2d");
const controlsCanvas = $("local-controls"), controls = controlsCanvas.getContext("2d");
const explicitStream = new URLSearchParams(location.search).get("view") === "stream";
const video = $("board-stream"), stage = $("native-stage");
let meta = null, cur = null, layout = null, selection = null;
let serial = Date.now() * 1000, pendingSerial = 0, lastAimSent = 0;
let pointer = null, aim = null, running = true, viewReady = false, polling = false;
let stream = null, sceneStream = null, sceneRenderer = null, imageRequest = null, browserMode = false, viewStarted = false;
let videoTimer = null, readySceneMatchId = null, sceneRetryTimer = null, sceneFailed = false;
const human = () => meta?.mode === "human";
const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function post(path, body = {}, signal) {
  const response = await fetch(BASE + path, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(body), signal });
  const data = await response.json();
  if (!response.ok) throw new Error(data.error || `HTTP ${response.status}`);
  return data;
}
function control(op, value) {
  return post(`/api/matches/${matchId}/control`, { op, ...(value === undefined ? {} : { value }) })
    .catch((error) => { $("feedback").textContent = error.message; });
}
const inputTransport = new ArenaInputTransport({
  send: (command, signal) => post(`/api/matches/${matchId}/input`, command, signal),
  onError: error => { $("feedback").textContent = error.message; drawControls(); }
});
function send(op, extra = {}) {
  if (!cur?.human) return;
  serial = Math.max(serial + 1, Date.now() * 1000);
  const command = { op, match_id: cur.human.match_id, serial, ...extra };
  const accepted = inputTransport.enqueue(command);
  if (accepted && ["select", "play", "cancel"].includes(op)) pendingSerial = serial;
  return accepted;
}
function showLoading(message, final = false) {
  $("loading").classList.add("show");
  $("loading-msg").textContent = message;
  $("loading").querySelector(".spinner").hidden = final;
  if (final) $("loading").querySelector("h2").textContent = "Match stopped";
}
function stop(message) {
  running = false; clearTimeout(sceneRetryTimer);
  stream?.close(); sceneStream?.close(); sceneRenderer?.destroy(); inputTransport.destroy();
  imageRequest?.abort();
  stopVideo();
  showLoading(message, true);
}
function stopVideo() {
  clearInterval(videoTimer); videoTimer = null;
  video.onload = video.onerror = null;
  video.removeAttribute("src");
}
function resize() {
  const rect = $("native-wrap").getBoundingClientRect();
  const width = Math.min(rect.width, rect.height * 540 / 900, 540);
  $("native-stage").style.width = `${width}px`;
  $("native-stage").style.height = `${width * 900 / 540}px`;
}
new ResizeObserver(resize).observe($("native-wrap"));
window.addEventListener("resize", resize);

function updateHandTargets() {
  const container = $("hand-targets");
  if (!human() || !layout || !cur?.hands[0]) {
    if (container.childElementCount) container.replaceChildren();
    return;
  }
  for (const card of cur.hands[0]) {
    let button = container.querySelector(`[data-slot="${card.slot}"]`);
    if (!button) {
      button = document.createElement("button");
      button.className = "native-card";
      button.dataset.slot = card.slot;
      button.onpointerdown = (event) => {
        event.preventDefault();
        selectSlot(Number(button.dataset.slot));
        pointer = { id: event.pointerId, fromCard: true };
      };
      button.onclick = (event) => { if (event.detail === 0) selectSlot(Number(button.dataset.slot)); };
      container.append(button);
    }
    const [x, y, w, h] = layout.cards[card.slot];
    const box = `${x / 540 * 100}% ${y / 900 * 100}% ${w / 540 * 100}% ${h / 900 * 100}%`;
    if (button.dataset.box !== box) {
      button.dataset.box = box;
      Object.assign(button.style, { left: `${x / 540 * 100}%`, top: `${y / 900 * 100}%`,
        width: `${w / 540 * 100}%`, height: `${h / 900 * 100}%` });
    }
    const label = `${card.name}, ${card.cost} elixir, key ${card.slot + 1}`;
    if (button.getAttribute("aria-label") !== label) button.setAttribute("aria-label", label);
    const pressed = String(selection?.slot === card.slot);
    if (button.getAttribute("aria-pressed") !== pressed) button.setAttribute("aria-pressed", pressed);
    const unavailable = !card.available || cur.status === "ended";
    if (button.disabled !== unavailable) button.disabled = unavailable;
  }
}
function onFrame(frame) {
  if (frame.status === "failed") { stop(frame.message); return; }
  if (frame.loading || frame.tick === undefined) { showLoading(frame.message || "Loading original engine..."); return; }
  if (frame.human && cur?.human?.match_id !== frame.human.match_id) {
    inputTransport.reset(frame.human.match_id); pendingSerial = 0; selection = null; aim = null; pointer = null;

  }
  if (cur && cur.match_number !== frame.match_number) {
    selection = null; aim = null; pointer = null;
    if (browserMode && readySceneMatchId !== frame.match_number) { viewReady = false; showLoading("Preparing the new battle..."); }
  }
  cur = frame;
  if (frame.human && frame.human.last_serial >= pendingSerial) selection = frame.human.selection;
  const pause = $("pause");
  if (pause.dataset.state !== frame.status) {
    pause.dataset.state = frame.status;
    const paused = frame.status === "paused";
    pause.textContent = paused ? "▶" : "❚❚";
    pause.setAttribute("aria-label", paused ? "Resume" : "Pause");
    pause.classList.toggle("on", paused);
  }
  const feedback = human() ? frame.human?.feedback || "" : frame.message || "";
  const feedbackNode = $("feedback");
  if (feedbackNode.dataset.t !== feedback) {
    feedbackNode.dataset.t = feedback;
    feedbackNode.textContent = feedback;
  }
  for (const button of $("speed").children) button.classList.toggle("on", Number(button.dataset.speed) === frame.speed);
  if (viewReady) $("loading").classList.remove("show");
  updateHandTargets(); drawControls();
}
function startView() {
  if (viewStarted || !running) return;
  viewStarted = true;
  browserMode = meta.renderer?.kind === "browser-scene" && !explicitStream;
  const alternate = `${BASE}/play.html?id=${encodeURIComponent(matchId)}${browserMode ? "&view=stream" : ""}`;
  $("view-mode").href = alternate;
  $("view-mode").hidden = meta.renderer?.kind !== "browser-scene";
  $("view-mode").textContent = browserMode ? "Stream view" : "Browser view";
  $("fallback-link").href = `${BASE}/play.html?id=${encodeURIComponent(matchId)}&view=stream`;
  canvas.hidden = !browserMode; fallbackCanvas.hidden = browserMode; controlsCanvas.hidden = !browserMode;
  if (!browserMode) { bootstrapView().then(ready => { if (ready && running) useVideoStream(); }); return; }
  try {
    sceneRenderer = new ArenaSceneRenderer(canvas, {
      textureUrl: id => `${BASE}/api/matches/${matchId}/textures/${id}.png`,
      alpha: false,
      onReady: packet => {
        layout = packet.layout; readySceneMatchId = packet.match_id; viewReady = true; $("fallback-link").hidden = true;
        if (cur) $("loading").classList.remove("show");
        updateHandTargets(); drawControls();
      },
      onError: error => {
        sceneFailed = true; clearTimeout(sceneRetryTimer); viewReady = false; showLoading(error.message, true); $("fallback-link").hidden = false;
        sceneStream?.close();
      }
    });
    connectScene();
  } catch (error) {
    viewReady = false; showLoading(error.message, true); $("fallback-link").hidden = false;
  }
}
function connectScene() {
    if (!running || sceneFailed) return;
    sceneStream?.close();
    sceneStream = new EventSource(`${BASE}/api/matches/${matchId}/scene`);
    sceneStream.addEventListener("scene", event => {
      const packet = JSON.parse(event.data); layout = packet.layout;
      sceneRenderer.push(packet); updateHandTargets(); drawControls();
    });
    sceneStream.addEventListener("error", event => {
      if (event.data) {
        sceneFailed = true; clearTimeout(sceneRetryTimer); viewReady = false; showLoading(JSON.parse(event.data).error, true); $("fallback-link").hidden = false; sceneStream.close();
      } else {
        if (!viewReady) showLoading("Preparing original graphics for your browser...");
        else $("feedback").textContent = "Reconnecting to the graphics...";
        // EventSource closes permanently on a preparing HTTP 503. Retry that
        // initial connection explicitly; ordinary network retries stay native.
        if (sceneStream.readyState === EventSource.CLOSED && running && !sceneFailed) {
          clearTimeout(sceneRetryTimer); sceneRetryTimer = setTimeout(connectScene, 500);
        }
      }
    });
}
function screenPoint(x, y) {
  const [left, top, width, height] = layout.field;
  return [left + x / layout.world[0] * width, top + (60 + (1 - y / layout.world[1]) * 768) / 828 * height];
}
let controlsDrawn = "";
function controlsKey() {
  // Scene packets arrive many times a second. Clearing the overlay when nothing
  // in it changed forces the browser to upload a full transparent canvas over
  // the game, which is enough to make the whole page feel sticky.
  if (!browserMode || !layout || !cur) return "";
  const marks = [];
  if (cur.status === "paused") marks.push("paused");
  const feedback = human() ? cur.human?.feedback || "" : "";
  if (feedback) marks.push("f:" + feedback);
  if (human() && selection) {
    marks.push("s:" + selection.slot + ":" + selection.card_id);
    if (aim) marks.push("a:" + aim.x + ":" + aim.y);
    const target = cur.human?.target;
    if (target && target.slot === selection.slot && target.card_id === selection.card_id) {
      marks.push("t:" + [target.x, target.y, target.land_x, target.land_y, target.valid, target.affordable, target.snaps, target.cost].join(":"));
      if (target.snaps) marks.push("z:" + JSON.stringify(cur.human.forbidden || []));
    }
  }
  if (marks.length && Array.isArray(layout.field)) marks.push("l:" + layout.field.join(","));
  return marks.join("|");
}
function drawControls() {
  const key = controlsKey();
  if (key === controlsDrawn) return;
  controlsDrawn = key;
  controls.clearRect(0, 0, 540, 900);
  if (!key) return;
  controls.font = "bold 12px sans-serif"; controls.textAlign = "center";
  const text = (label, x, y, size = 12) => {
    controls.font = `bold ${size}px sans-serif`; controls.lineWidth = 3;
    controls.strokeStyle = "#111c27"; controls.strokeText(label, x, y);
    controls.fillStyle = "white"; controls.fillText(label, x, y);
  };
  if (human()) {
    for (const [slot, [x, y, w, h]] of layout.cards.entries()) {
      if (selection?.slot === slot) {
        controls.strokeStyle = "#62e8ff"; controls.lineWidth = 3;
        controls.strokeRect(x - 1.5, y - 1.5, w + 3, h + 3);
      }
    }
    if (selection && aim) {
      const t = cur.human?.target;
      const checked = t && t.slot === selection.slot && t.card_id === selection.card_id && t.x === aim.x && t.y === aim.y ? t : null;
      if (checked?.snaps) {
        controls.fillStyle = "#ff000066";
        for (const [x0, y0, x1, y1] of cur.human.forbidden || []) {
          const [ax, ay] = screenPoint(x0, y1), [bx, by] = screenPoint(x1, y0);
          controls.fillRect(ax, ay, bx - ax, by - ay);
        }
      }
      const [x, y] = screenPoint(checked?.land_x ?? aim.x, checked?.land_y ?? aim.y);
      controls.strokeStyle = checked ? !checked.valid ? "#ff5b5a" : !checked.affordable ? "#ffc247" : "#6cff80" : "#62e8ff";
      controls.lineWidth = 2; controls.beginPath(); controls.arc(x, y, 12, 0, Math.PI * 2);
      controls.moveTo(x - 17, y); controls.lineTo(x + 17, y); controls.moveTo(x, y - 17); controls.lineTo(x, y + 17); controls.stroke();
      if (checked && (!checked.valid || !checked.affordable)) text(!checked.valid ? "Cannot place here" : `Need ${checked.cost} elixir`, x, y + 30);
    }
    if (cur.human?.feedback) text(cur.human.feedback, 270, 110);
  }
  if (cur.status === "paused") {
    controls.fillStyle = "#131f2ef0"; controls.fillRect(170, 425, 200, 50);
    text("PAUSED", 270, 458, 24);
  }
}
// Read-only diagnostics for local profiling and QA.
window.arenaDiagnostics = () => ({mode: browserMode ? "browser" : "stream", scene: sceneRenderer?.stats, input: inputTransport.stats, tick: cur?.tick, status: cur?.status});

async function bootstrapView() {
  // One frame up front: instant picture, plus the input layout header.
  while (running) {
    try {
      imageRequest = new AbortController();
      const response = await fetch(`${BASE}/api/matches/${matchId}/view`, { signal: imageRequest.signal });
      if (!response.ok) {
        const data = await response.json();
        if (response.status === 404) { stop(data.error); return false; }
      } else {
        const bitmap = await createImageBitmap(await response.blob());
        if (!running) { bitmap.close(); return false; }
        ctx.drawImage(bitmap, 0, 0, 540, 900); bitmap.close();
        layout = JSON.parse(response.headers.get("X-Arena-Layout")); updateHandTargets();
        viewReady = true;
        if (cur) $("loading").classList.remove("show");
        return true;
      }
    } catch { if (!running) return false; }
    await delay(350);
  }
  return false;
}
function useVideoStream() {
  // One multipart connection delivers every new frame the renderer produces;
  // frame rate is not divided by a request round trip per frame.
  const ready = () => {
    if (!running || polling || !video.naturalWidth) return;
    video.hidden = false; fallbackCanvas.hidden = true; canvas.hidden = true;
    viewReady = true;
    clearInterval(videoTimer); videoTimer = null;
    if (cur) $("loading").classList.remove("show");
  };
  video.onload = ready;
  video.onerror = () => { if (running) pollImages(); };
  const started = performance.now();
  // Some browsers postpone the load event for an endless multipart response.
  videoTimer = setInterval(() => {
    ready();
    if (video.hidden && performance.now() - started > 8000) pollImages();
  }, 100);
  video.src = `${BASE}/api/matches/${matchId}/video`;
}
async function pollImages() {
  if (polling || !running) return;
  polling = true;
  stopVideo();
  video.hidden = true; fallbackCanvas.hidden = false; canvas.hidden = true;
  let after = 0;
  while (running) {
    try {
      imageRequest = new AbortController();
      const response = await fetch(`${BASE}/api/matches/${matchId}/view?after=${after}`, { signal: imageRequest.signal });
      if (!response.ok) {
        const data = await response.json();
        if (response.status === 404) { stop(data.error); return; }
        if (!viewReady) showLoading(data.error || "Preparing original game view...");
        await delay(350);
        continue;
      }
      const next = Number(response.headers.get("X-Arena-Sequence"));
      const newLayout = JSON.parse(response.headers.get("X-Arena-Layout"));
      const bitmap = await createImageBitmap(await response.blob());
      ctx.drawImage(bitmap, 0, 0, 540, 900);
      bitmap.close();
      after = next;
      if (!layout) { layout = newLayout; updateHandTargets(); }
      viewReady = true;
      if (cur) $("loading").classList.remove("show");
    } catch (error) {
      if (!running) return;
      $("feedback").textContent = `Reconnecting to game view: ${error.message}`;
      await delay(500);
    }
  }
}
async function connect() {
  if (!matchId) { stop("No match selected. Go back to the lobby."); return; }
  try {
    const response = await fetch(`${BASE}/api/matches/${matchId}`);
    const summary = await response.json();
    if (!response.ok) { stop(summary.error); return; }
    if (summary.error || summary.status === "closed") { stop(summary.error || summary.message); return; }
    stream = new EventSource(`${BASE}/api/matches/${matchId}/stream`);
    stream.addEventListener("meta", (event) => {
      meta = JSON.parse(event.data);
      $("match-title").textContent = meta.labels.join(" vs ");
      document.title = `${meta.labels.join(" vs ")} · Arena`;
      $("speed").hidden = human();
      $("speed").replaceChildren(...(human() ? [] : [0.5, 1, 2, 4].map((value) => {
        const button = document.createElement("button");
        button.className = "icon-btn"; button.textContent = `${value}×`; button.dataset.speed = value;
        button.onclick = () => control("speed", value);
        return button;
      })));
      updateHandTargets();
      startView();
    });
    stream.addEventListener("frame", (event) => onFrame(JSON.parse(event.data)));
    stream.addEventListener("status", (event) => {
      const status = JSON.parse(event.data);
      if (status.error) stop(status.error);
    });
    stream.addEventListener("closed", () => stop(cur?.message || "This match has closed."));
    stream.onerror = () => { $("feedback").textContent = "Reconnecting to the match..."; };

  } catch (error) { stop(error.message); }
}
function toWorld(event) {
  if (!layout) return null;
  const rect = stage.getBoundingClientRect();
  const px = (event.clientX - rect.left) * 540 / rect.width;
  const py = (event.clientY - rect.top) * 900 / rect.height;
  const [left, top, width, height] = layout.field;
  if (px < left || px > left + width || py < top || py > top + height) return null;
  const x = (px - left) / width * layout.world[0];
  const y = (1 - ((py - top) / height * 828 - 60) / 768) * layout.world[1];
  return x >= 0 && x <= layout.world[0] && y >= 0 && y <= layout.world[1] ? { x: Math.round(x), y: Math.round(y) } : null;
}
function selectSlot(slot) {
  if (!human() || cur?.status === "ended") return;
  const card = cur?.hands[0]?.find((c) => c.slot === slot && c.available);
  if (!card) return;
  selection = { slot: card.slot, card_id: card.card_id };
  send("select", selection);
  updateHandTargets(); drawControls();
}
function sendAim(force = false) {
  if (!selection || !aim || !human()) return;
  if (!force && performance.now() - lastAimSent < 80) return;
  lastAimSent = performance.now();
  send("aim", { ...selection, ...aim });
}
function cancel() { selection = null; aim = null; send("cancel"); updateHandTargets(); drawControls(); }
stage.addEventListener("pointerdown", (event) => {
  if (!human() || !selection || cur?.status === "ended" || event.button === 2) return;
  const target = toWorld(event);
  if (!target) return;
  event.preventDefault();
  aim = target; pointer = { id: event.pointerId, fromCard: false }; sendAim(true); drawControls();
});
window.addEventListener("pointermove", (event) => {
  if (!human() || !selection) return;
  if (pointer && pointer.id !== event.pointerId) return;
  if (!pointer && event.pointerType !== "mouse") return;
  aim = toWorld(event);
  sendAim(); drawControls();
});
window.addEventListener("pointerup", (event) => {
  if (!pointer || pointer.id !== event.pointerId) return;
  pointer = null;
  aim = toWorld(event);
  if (aim && selection) send("play", { ...selection, ...aim });
  drawControls();
});
window.addEventListener("pointercancel", () => { pointer = null; aim = null; drawControls(); });
stage.addEventListener("contextmenu", (event) => { event.preventDefault(); cancel(); });
window.addEventListener("keydown", (event) => {
  if (event.target.closest("input,select,textarea")) return;
  if (event.key === " " && !event.target.closest("button,a")) { event.preventDefault(); control("pause"); }
  else if (event.key.toLowerCase() === "r") control("reset");
  else if (event.key.toLowerCase() === "q") $("end-match").click();
  else if (event.key === "Escape") cancel();
  else if (/^[1-4]$/.test(event.key)) selectSlot(Number(event.key) - 1);
});
$("pause").onclick = () => control("pause");
$("rematch").onclick = () => control("reset");
$("end-match").onclick = async () => {
  try { await post(`/api/matches/${matchId}/close`); running = false; clearTimeout(sceneRetryTimer); stream?.close(); sceneStream?.close(); sceneRenderer?.destroy(); inputTransport.destroy(); imageRequest?.abort(); stopVideo(); location.href = `${BASE}/`; }
  catch (error) { $("feedback").textContent = error.message; }
};
$("fullscreen").onclick = () => {
  const action = document.fullscreenElement ? document.exitFullscreen() : $("native-wrap").requestFullscreen?.();
  action?.catch(() => {});
};
window.addEventListener("pagehide", () => { running = false; clearTimeout(sceneRetryTimer); stream?.close(); sceneStream?.close(); sceneRenderer?.destroy(); inputTransport.destroy(); imageRequest?.abort(); stopVideo(); });
window.addEventListener("pageshow", event => { if (event.persisted) location.reload(); });
resize();
connect();
