"use strict";
// Pixels come from the desktop spectator's original-asset Rust Painter.
// Pointer/touch input uses its exported ControlLayout and camera projection.
const $ = (id) => document.getElementById(id);
const BASE = new URL(".", document.currentScript.src).pathname.replace(/\/$/, "");
const matchId = new URLSearchParams(location.search).get("id");
const canvas = $("board"), ctx = canvas.getContext("2d");
const video = $("board-stream"), stage = $("native-stage");
let meta = null, cur = null, layout = null, selection = null;
let serial = Date.now() * 1000, pendingSerial = 0, lastAimSent = 0;
let pointer = null, aim = null, running = true, viewReady = false, polling = false;
let stream = null, imageRequest = null, commandQueue = Promise.resolve();
let videoTimer = null;
const human = () => meta?.mode === "human";
const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function post(path, body = {}) {
  const response = await fetch(BASE + path, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(body) });
  const data = await response.json();
  if (!response.ok) throw new Error(data.error || `HTTP ${response.status}`);
  return data;
}
function control(op, value) {
  return post(`/api/matches/${matchId}/control`, { op, ...(value === undefined ? {} : { value }) })
    .catch((error) => { $("feedback").textContent = error.message; });
}
function send(op, extra = {}) {
  if (!cur?.human) return;
  serial = Math.max(serial + 1, Date.now() * 1000);
  const command = { op, match_id: cur.human.match_id, serial, ...extra };
  if (["select", "play", "cancel"].includes(op)) pendingSerial = serial;
  // Preserve select → aim → play ordering on a slower phone connection.
  commandQueue = commandQueue.then(() => post(`/api/matches/${matchId}/input`, command))
    .catch((error) => { $("feedback").textContent = error.message; });
}
function showLoading(message, final = false) {
  $("loading").classList.add("show");
  $("loading-msg").textContent = message;
  $("loading").querySelector(".spinner").hidden = final;
  if (final) $("loading").querySelector("h2").textContent = "Match stopped";
}
function stop(message) {
  running = false;
  stream?.close();
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
  if (!human() || !layout || !cur?.hands[0]) { container.replaceChildren(); return; }
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
    Object.assign(button.style, { left: `${x / 540 * 100}%`, top: `${y / 900 * 100}%`,
      width: `${w / 540 * 100}%`, height: `${h / 900 * 100}%` });
    button.setAttribute("aria-label", `${card.name}, ${card.cost} elixir, key ${card.slot + 1}`);
    button.setAttribute("aria-pressed", String(selection?.slot === card.slot));
    button.disabled = !card.available || cur.status === "ended";
  }
}
function onFrame(frame) {
  if (frame.status === "failed") { stop(frame.message); return; }
  if (frame.loading || frame.tick === undefined) { showLoading(frame.message || "Loading original engine..."); return; }
  if (cur && cur.match_number !== frame.match_number) { selection = null; aim = null; }
  cur = frame;
  if (frame.human && frame.human.last_serial >= pendingSerial) selection = frame.human.selection;
  $("pause").textContent = frame.status === "paused" ? "▶" : "❚❚";
  $("pause").setAttribute("aria-label", frame.status === "paused" ? "Resume" : "Pause");
  $("pause").classList.toggle("on", frame.status === "paused");
  $("feedback").textContent = human() ? frame.human?.feedback || "" : frame.message;
  for (const button of $("speed").children) button.classList.toggle("on", Number(button.dataset.speed) === frame.speed);
  if (viewReady) $("loading").classList.remove("show");
  updateHandTargets();
}
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
    video.hidden = false; canvas.hidden = true;
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
  video.hidden = true; canvas.hidden = false;
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
    });
    stream.addEventListener("frame", (event) => onFrame(JSON.parse(event.data)));
    stream.addEventListener("status", (event) => {
      const status = JSON.parse(event.data);
      if (status.error) stop(status.error);
    });
    stream.addEventListener("closed", () => stop(cur?.message || "This match has closed."));
    stream.onerror = () => { $("feedback").textContent = "Reconnecting to the match..."; };
    bootstrapView().then((ready) => { if (ready && running) useVideoStream(); });
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
  updateHandTargets();
}
function sendAim(force = false) {
  if (!selection || !aim || !human()) return;
  if (!force && performance.now() - lastAimSent < 80) return;
  lastAimSent = performance.now();
  send("aim", { ...selection, ...aim });
}
function cancel() { selection = null; aim = null; send("cancel"); updateHandTargets(); }
stage.addEventListener("pointerdown", (event) => {
  if (!human() || !selection || cur?.status === "ended" || event.button === 2) return;
  const target = toWorld(event);
  if (!target) return;
  event.preventDefault();
  aim = target; pointer = { id: event.pointerId, fromCard: false }; sendAim(true);
});
window.addEventListener("pointermove", (event) => {
  if (!human() || !selection) return;
  if (pointer && pointer.id !== event.pointerId) return;
  if (!pointer && event.pointerType !== "mouse") return;
  aim = toWorld(event);
  sendAim();
});
window.addEventListener("pointerup", (event) => {
  if (!pointer || pointer.id !== event.pointerId) return;
  pointer = null;
  aim = toWorld(event);
  if (aim && selection) send("play", { ...selection, ...aim });
});
window.addEventListener("pointercancel", () => { pointer = null; aim = null; });
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
  try { await post(`/api/matches/${matchId}/close`); running = false; stream?.close(); imageRequest?.abort(); stopVideo(); location.href = `${BASE}/`; }
  catch (error) { $("feedback").textContent = error.message; }
};
$("fullscreen").onclick = () => {
  const action = document.fullscreenElement ? document.exitFullscreen() : $("native-wrap").requestFullscreen?.();
  action?.catch(() => {});
};
window.addEventListener("pagehide", () => { running = false; stream?.close(); imageRequest?.abort(); stopVideo(); });
resize();
connect();
