"use strict";
// Lobby: choose models and decks, then start a match through the JSON API.

const $ = (id) => document.getElementById(id);
const BASE = new URL(".", document.currentScript.src).pathname.replace(/\/$/, "");
const state = {
  mode: "human",
  catalog: null,
  cards: {},
  sides: {
    red: { model: null, deck: null, deckTouched: false },
    blue: { model: null, deck: null, deckTouched: false },
  },
  customDecks: JSON.parse(localStorage.getItem("arena.customDecks") || "[]"),
  builderFor: null,
  builderCards: [],
};

$("lab-link").href = `${location.protocol}//${location.hostname}:8801/`;

async function api(path, options = {}) {
  const response = await fetch(BASE + path, {
    headers: { "Content-Type": "application/json" },
    ...options,
    body: options.body ? JSON.stringify(options.body) : undefined,
  });
  const data = await response.json().catch(() => ({}));
  if (!response.ok) throw new Error(data.error || `HTTP ${response.status}`);
  return data;
}

function el(tag, attrs = {}, ...children) {
  const node = document.createElement(tag);
  for (const [key, value] of Object.entries(attrs)) {
    if (key === "class") node.className = value;
    else if (key.startsWith("on")) node.addEventListener(key.slice(2), value);
    else if (value !== undefined && value !== null) node.setAttribute(key, value);
  }
  for (const child of children.flat()) {
    if (child === null || child === undefined) continue;
    node.append(child instanceof Node ? child : document.createTextNode(String(child)));
  }
  return node;
}

function deckById(id) {
  return state.catalog.presets.find((d) => d.id === id) || state.customDecks.find((d) => d.id === id);
}

function averageElixir(cards) {
  return (cards.reduce((sum, c) => sum + (state.cards[c]?.elixir || 0), 0) / cards.length).toFixed(1);
}

function modelById(id) {
  return state.catalog.models.find((m) => m.id === id);
}

// ---------------- models ----------------
function renderModels(container, sideKey) {
  const side = state.sides[sideKey];
  container.replaceChildren();
  for (const model of state.catalog.models) {
    const kindLabel = { rl: "RL", imitation: "Imitation", baseline: "Baseline" }[model.kind] || model.kind;
    const card = el("button", {
      class: `model${side.model === model.id ? " selected" : ""}${model.available ? "" : " unavailable"}`,
      type: "button",
      onclick: () => {
        if (!model.available) return;
        side.model = model.id;
        if (!side.deckTouched && model.trained_deck) side.deck = model.trained_deck;
        render();
      },
    },
    el("div", { class: "name" }, model.recommended ? el("span", { class: "star" }, "★") : null, model.name,
      el("span", { class: `badge ${model.kind}` }, kindLabel)),
    el("div", { class: "meta" }, model.trained_deck_name ? `Trained on ${model.trained_deck_name}` :
      model.kind === "imitation" ? "Learned from human replays · any deck" : "Any deck",
    model.available ? "" : " · checkpoint missing"),
    el("div", { class: "summary" }, model.summary),
    model.result ? el("div", { class: "result" }, model.result) : null);
    container.append(card);
  }
}

// ---------------- decks ----------------
function renderDecks(container, sideKey, noteEl) {
  const side = state.sides[sideKey];
  const model = sideKey === "blue" && state.mode === "human" ? null : modelById(side.model);
  container.replaceChildren();
  const all = [...state.catalog.presets.map((d) => ({ ...d, custom: false })),
    ...state.customDecks.map((d) => ({ ...d, custom: true }))];
  for (const deck of all) {
    const trained = model && model.trained_deck === deck.id;
    const node = el("button", {
      class: `deck${side.deck === deck.id ? " selected" : ""}`, type: "button",
      onclick: () => { side.deck = deck.id; side.deckTouched = true; render(); },
    },
    el("div", { class: "title" }, el("span", {}, deck.name),
      trained ? el("span", { class: "tag" }, "★ trained") : deck.custom ? el("span", { class: "tag custom" }, "custom") : null),
    el("div", { class: "sub" }, el("span", {}, `${averageElixir(deck.cards)} avg elixir`),
      deck.archetype ? el("span", {}, `· ${deck.archetype}`) : null),
    el("div", { class: "grid" }, deck.cards.map((c) => el("img", {
      src: state.cards[c]?.art, alt: state.cards[c]?.name || c, title: state.cards[c]?.name || c, loading: "lazy",
    }))));
    if (deck.custom) {
      node.append(el("span", {
        class: "del", title: "Delete this custom deck", role: "button",
        onclick: (event) => {
          event.stopPropagation();
          state.customDecks = state.customDecks.filter((d) => d.id !== deck.id);
          saveCustom();
          for (const s of Object.values(state.sides)) if (s.deck === deck.id) s.deck = state.catalog.presets[0].id;
          render();
        },
      }, "×"));
    }
    container.append(node);
  }
  container.append(el("button", { class: "deck add", type: "button", onclick: () => openBuilder(sideKey) }, "+ Build a custom deck"));
  const deck = deckById(side.deck);
  const isPreset = state.catalog.presets.some((d) => d.id === side.deck);
  if (noteEl) {
    noteEl.className = "note";
    noteEl.textContent = "";
    if (model && model.trained_deck && deck && model.trained_deck !== deck.id) {
      noteEl.className = "note warn";
      noteEl.textContent = `${model.name} was trained on ${model.trained_deck_name}; performance with this deck is unverified.`;
    } else if (!isPreset && deck) {
      noteEl.className = "note warn";
      noteEl.textContent = "Custom deck: runs in the original engine. Specialist models may play less well outside their training deck.";
    }
  }
}

function saveCustom() {
  localStorage.setItem("arena.customDecks", JSON.stringify(state.customDecks));
}

// ---------------- deck builder ----------------
function openBuilder(sideKey) {
  state.builderFor = sideKey;
  const current = deckById(state.sides[sideKey].deck);
  state.builderCards = current ? [...current.cards] : [];
  $("card-search").value = "";
  $("builder").classList.add("open");
  $("backdrop").classList.add("open");
  renderBuilder();
}

function closeBuilder() {
  $("builder").classList.remove("open");
  $("backdrop").classList.remove("open");
}

function renderBuilder() {
  const chosen = state.builderCards;
  const slots = $("slots");
  slots.replaceChildren();
  for (let i = 0; i < 8; i++) {
    const card = state.cards[chosen[i]];
    slots.append(el("div", {
      class: "slot", title: card ? `Remove ${card.name}` : "Empty",
      onclick: () => { if (card) { chosen.splice(i, 1); renderBuilder(); } },
    }, card ? el("img", { src: card.art, alt: card.name }) : null, card ? el("span", { class: "cost" }, card.elixir) : null));
  }
  const query = $("card-search").value.trim().toLowerCase();
  const model = modelById(state.sides[state.builderFor]?.model);
  const trainedCards = new Set((model && deckById(model.trained_deck)?.cards) || []);
  const pool = $("pool");
  pool.replaceChildren();
  for (const card of state.catalog.cards) {
    if (query && !card.name.toLowerCase().includes(query)) continue;
    const inDeck = chosen.includes(card.id);
    pool.append(el("button", {
      class: `pcard${inDeck ? " in" : ""}${trainedCards.has(card.id) ? " trained" : ""}`, type: "button",
      title: `${card.name} · ${card.elixir} elixir${card.rarity ? ` · ${card.rarity}` : ""}`,
      onclick: () => {
        if (inDeck) chosen.splice(chosen.indexOf(card.id), 1);
        else if (chosen.length < 8) chosen.push(card.id);
        renderBuilder();
      },
    }, el("img", { src: card.art, alt: card.name, loading: "lazy" }), el("span", { class: "cost" }, card.elixir),
    el("div", { class: "nm" }, card.name)));
  }
  $("builder-info").textContent = chosen.length === 8
    ? `${averageElixir(chosen)} average elixir. ★ = in the selected model's training deck.`
    : `Pick ${8 - chosen.length} more card${8 - chosen.length === 1 ? "" : "s"}.`;
  $("builder-save").disabled = chosen.length !== 8;
}

$("card-search").addEventListener("input", renderBuilder);
$("builder-close").addEventListener("click", closeBuilder);
$("backdrop").addEventListener("click", closeBuilder);
$("builder-save").addEventListener("click", () => {
  const cards = [...state.builderCards];
  const existing = state.customDecks.find((d) => d.cards.slice().sort().join() === cards.slice().sort().join());
  const deck = existing || { id: `custom-${Date.now().toString(36)}`, name: `Custom ${state.customDecks.length + 1}`, cards };
  if (!existing) { state.customDecks.push(deck); saveCustom(); }
  const side = state.sides[state.builderFor];
  side.deck = deck.id;
  side.deckTouched = true;
  closeBuilder();
  render();
});

// ---------------- mode, active matches, start ----------------
for (const button of document.querySelectorAll(".tabs button")) {
  button.addEventListener("click", () => {
    state.mode = button.dataset.mode;
    document.querySelectorAll(".tabs button").forEach((b) => b.classList.toggle("active", b === button));
    render();
  });
}

function render() {
  const watch = state.mode === "watch";
  $("red-title").textContent = watch ? "Red model" : "Opponent";
  $("red-deck-title").textContent = watch ? "Red deck" : "Opponent's deck";
  $("blue-deck-title").textContent = watch ? "Blue deck" : "Your deck";
  $("blue-deck-hint").textContent = watch ? "The blue model plays from the bottom." : "You play blue, from the bottom.";
  $("blue-model-section").style.display = watch ? "" : "none";
  $("start").textContent = watch ? "Watch match" : "Play";
  renderModels($("red-models"), "red");
  renderDecks($("red-decks"), "red", $("red-note"));
  if (watch) renderModels($("blue-models"), "blue");
  renderDecks($("blue-decks"), "blue", $("blue-note"));
  prefetchSelectedDecks();
}

let prefetchToken = 0;
function selectedNatives() {
  const natives = [];
  for (const side of [state.sides.red, state.sides.blue]) {
    const deck = deckById(side.deck);
    if (!deck) return null;
    for (const id of deck.cards) {
      const native = state.cards[id]?.native;
      if (native) natives.push(native);
    }
  }
  return natives;
}
function prefetchFetch(url) {
  return fetch(url, {priority: "low", credentials: "same-origin"});
}
async function prefetchSelectedDecks() {
  if (typeof planPackFiles !== "function" || typeof resolveAssetUrl !== "function") return;
  const natives = selectedNatives();
  if (!natives || !natives.length) return;
  const key = natives.slice().sort().join("|");
  if (key === state.prefetchKey) return;
  state.prefetchKey = key;
  const token = ++prefetchToken;
  const manifestUrl = `${BASE}/assets/manifest.json`;
  try {
    const first = await prefetchFetch(manifestUrl);
    if (!first.ok || token !== prefetchToken) return;
    let manifest = await first.json();
    let resolved = manifestUrl;
    if (manifest.current && !manifest.v) {
      resolved = resolveAssetUrl(manifestUrl, manifest.current);
      const second = await prefetchFetch(resolved);
      if (!second.ok || token !== prefetchToken) return;
      manifest = await second.json();
    }
    const plan = planPackFiles(manifest, natives);
    const jobs = plan.files.map((file) => prefetchFetch(resolveAssetUrl(resolved, file.url)));
    if (plan.wasm) jobs.push(prefetchFetch(resolveAssetUrl(resolved, plan.wasm.url)));
    await Promise.all(jobs);
  } catch { /* packs are optional until a match starts */ }
}

async function refreshActive() {
  try {
    const { matches } = await api("/api/matches");
    $("active-section").style.display = matches.length ? "" : "none";
    $("active").replaceChildren(...matches.map((m) => el("div", { class: "match-row" },
      el("div", { class: "grow" }, `${m.labels[0]} vs ${m.labels[1]}`,
        el("div", { class: "note", style: "margin:0" }, `${m.mode === "human" ? "You playing" : "Watching"} · ${m.status}`)),
      el("a", { class: "btn primary", href: `${BASE}/play.html?id=${m.id}` }, "Open"),
      el("button", { class: "btn danger", onclick: async () => { await api(`/api/matches/${m.id}/close`, { method: "POST" }); refreshActive(); } }, "End"))));
  } catch (error) { /* server restarting; try again later */ }
}

$("start").addEventListener("click", async () => {
  const errorBox = $("error");
  errorBox.style.display = "none";
  const red = state.sides.red;
  const blue = state.sides.blue;
  const deckCards = (id) => deckById(id)?.cards;
  const body = {
    sides: [
      state.mode === "human" ? { controller: "human", deck: deckCards(blue.deck) }
        : { controller: blue.model, deck: deckCards(blue.deck) },
      { controller: red.model, deck: deckCards(red.deck) },
    ],
  };
  const seed = $("seed").value.trim();
  if (seed) body.seed = Number(seed);
  $("start").disabled = true;
  $("start").textContent = "Starting...";
  try {
    const match = await api("/api/matches", { method: "POST", body });
    location.href = `${BASE}/play.html?id=${match.id}`;
  } catch (error) {
    errorBox.textContent = error.message;
    errorBox.style.display = "block";
    $("start").disabled = false;
    render();
  }
});

(async function init() {
  try {
    state.catalog = await api("/api/catalog");
  } catch (error) {
    $("error").textContent = `Cannot reach the Arena server: ${error.message}`;
    $("error").style.display = "block";
    return;
  }
  for (const card of state.catalog.cards) { card.art = BASE + card.art; state.cards[card.id] = card; }
  $("lab-link").hidden = !!state.catalog.public;
  // Drop saved custom decks that reference cards no longer offered.
  state.customDecks = state.customDecks.filter((d) => d.cards.length === 8 && d.cards.every((c) => state.cards[c]));
  const available = state.catalog.models.filter((m) => m.available);
  const first = available.find((m) => m.recommended) || available[0];
  const second = available.find((m) => m.id !== first?.id && m.recommended) || available[1] || first;
  state.sides.red.model = first?.id;
  state.sides.red.deck = first?.trained_deck || state.catalog.presets[0].id;
  state.sides.blue.model = second?.id;
  state.sides.blue.deck = state.catalog.presets.find((d) => d.id === "log-bait")?.id || state.catalog.presets[0].id;
  render();
  refreshActive();
  setInterval(refreshActive, 5000);
})();
