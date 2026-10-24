/* Run with: node --test webapps/arena/test_scene_renderer.cjs */
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {ArenaSceneRenderer, SoftwarePainter, interpolateDraws, collectTextures, galleryFileUrl} = require('./static/scene-renderer.js');
function canvas() {
  const context = {createImageData: (w, h) => ({data: new Uint8ClampedArray(w * h * 4)}), putImageData() {}};
  return {width: 0, height: 0, getContext: kind => kind === '2d' ? context : null, addEventListener() {}, removeEventListener() {}};
}
function sprite(overrides = {}) {return {id: 'troop-12', kind: 'sprite', tex: 'a', m: [1, 0, 0, 1, 0, 0], alpha: 1, blend: 'normal', ...overrides};}
function packet(sequence = 1, overrides = {}) {return {v: 1, sequence, tick: sequence, display_tick: sequence, match_id: 1, width: 2, height: 1, clear: [100, 100, 100, 255], draws: [sprite()], textures: [], ...overrides};}
function paint(draws, tex = [200, 80, 20, 128], clear = [100, 100, 100, 255]) {
  const painter = new SoftwarePainter(canvas());
  painter.paint(packet(1, {draws, clear}), draws, new Map([['a', {resource: {width: 1, height: 1, pixels: new Uint8ClampedArray(tex)}}]]));
  return [...painter.output.data];
}
test('normal, additive, premultiplied additive and screen preserve their distinct alpha semantics', () => {
  assert.deepEqual(paint([sprite()]).slice(0, 4), [150, 89, 59, 255]);
  assert.deepEqual(paint([sprite({blend: 'add'})]).slice(0, 4), [200, 140, 110, 255]);
  assert.deepEqual(paint([sprite({blend: 'premultiplied-add'})]).slice(0, 4), [255, 180, 120, 255]);
  assert.deepEqual(paint([sprite({blend: 'premultiplied-screen'})]).slice(0, 4), [221, 148, 112, 255]);
});
test('color offsets clamp the original channels before alpha multiplication', () => {
  assert.deepEqual(paint([sprite({add: [100, -200, 250]})]).slice(0, 4), [177, 49, 177, 255]);
});
test('literal replacement preserves source RGBA and clears transparent geometry', () => {
  assert.deepEqual(paint([sprite({replace: true})]).slice(0, 4), [200, 80, 20, 128]);
  assert.deepEqual(paint([{kind: 'rect', x: 0, y: 0, w: 1, h: 1, color: [30, 40, 50, 0], replace: true}]).slice(0, 4), [30, 40, 50, 0]);
});
test('affine Canvas ordering and alpha-weighted filtering avoid transparent color fringes', () => {
  assert.deepEqual(paint([sprite({m: [1, 0, 0, 1, 1, 0]})]), [100, 100, 100, 255, 150, 89, 59, 255]);
  const transformed = paint([sprite({m: [1, 0, 0, 1, .5, 0]})]);
  assert.deepEqual(transformed.slice(0, 4), transformed.slice(4, 8));
  assert.deepEqual(transformed.slice(0, 4), [125, 94, 79, 255]);
});
test('overlapping silhouettes combine by MAX rather than accumulating darkness', () => {
  const part = sprite({alpha: .5});
  const once = paint([{kind: 'shadow', draws: [part]}]);
  const twice = paint([{kind: 'shadow', draws: [part, part]}]);
  assert.deepEqual(once, twice);
  assert.deepEqual(once.slice(0, 4), [75, 75, 100, 255]);
});
test('soft ellipse shadows retain the original flat-center falloff', () => {
  const pixels = paint([{kind: 'shadow', draws: [{kind: 'ellipse', cx: .5, cy: .5, rx: 1, ry: 1, alpha: .5}]}]);
  assert.deepEqual(pixels.slice(0, 4), [50, 50, 100, 255]);
  assert.deepEqual(pixels.slice(4, 8), [100, 100, 100, 255]);
});
test('a texture change without anchors keeps the previous crop instead of lerping it', () => {
  const older = [sprite(), sprite({id: 'removed'})];
  const newer = [sprite({id: 'new', m: [1, 0, 0, 1, 99, 0]}), sprite({tex: 'b', m: [2, 0, 0, 2, 10, 20]})];
  const middle = interpolateDraws(older, newer, .5);
  assert.equal(middle[0], older[0]);
  assert.deepEqual(middle[0].m, [1, 0, 0, 1, 0, 0]);
  assert.equal(middle[0].tex, 'a');
  assert.equal(middle[1], older[1]);
  assert.equal(interpolateDraws(older, newer, 1), newer);
  assert.deepEqual(older[0].m, [1, 0, 0, 1, 0, 0]);
});
test('different textures move anchor only, missing anchors hold the pose, and the same texture lerps m', () => {
  const older = [sprite({m: [1, 0, 0, 2, 100, 200], anchor: [120, 230]})];
  const newer = [sprite({tex: 'b', m: [4, 1, 0, 9, 0, 0], anchor: [140, 250]})];
  const moved = interpolateDraws(older, newer, .5);
  assert.equal(moved[0].tex, 'a');
  assert.deepEqual(moved[0].m, [1, 0, 0, 2, 110, 210]);
  assert.deepEqual(moved[0].anchor, [130, 240]);
  const missing = interpolateDraws([sprite({anchor: [4, 6]})], [sprite({tex: 'b', m: [3, 0, 0, 3, 9, 9]})], .5);
  assert.equal(missing[0].tex, 'a');
  assert.deepEqual(missing[0].m, [1, 0, 0, 1, 0, 0]);
  const omitted = interpolateDraws([sprite()], [sprite({tex: 'b', m: [8, 0, 0, 8, 40, 40], anchor: [10, 10]})], .5);
  assert.equal(omitted[0].tex, 'a');
  assert.deepEqual(omitted[0].m, [1, 0, 0, 1, 0, 0]);
  const same = interpolateDraws([sprite({m: [1, 0, 0, 1, 0, 0]})], [sprite({m: [1, 0, 0, 1, 100, 40]})], .25);
  assert.equal(same[0].tex, 'a');
  assert.deepEqual(same[0].m, [1, 0, 0, 1, 25, 10]);
  const shadow = (tex, anchor, m) => ({kind: 'shadow', id: 'shadow-1', draws: [sprite({id: 'shadow-1-part', tex, anchor, m})]});
  const nested = interpolateDraws([shadow('a', [10, 20], [1, 0, 0, 1, 3, 4])], [shadow('b', [30, 50], [9, 0, 0, 9, 0, 0])], .5);
  assert.equal(nested[0].draws[0].tex, 'a');
  assert.deepEqual(nested[0].draws[0].m, [1, 0, 0, 1, 13, 19]);
});
test('unknown commands, invalid matrices and unsafe texture IDs are explicit errors', () => {
  assert.throws(() => collectTextures([sprite({tex: '../secret'})]), /Invalid scene sprite/);
  assert.throws(() => collectTextures([sprite({m: [1, 0, 0, 1, Infinity, 0]})]), /Invalid scene sprite/);
  assert.throws(() => collectTextures([sprite({anchor: [Number.NaN, 1]})]), /Invalid scene sprite/);
  assert.throws(() => collectTextures([sprite({anchor: [1]})]), /Invalid scene sprite/);
  assert.throws(() => collectTextures([{kind: 'shadow', draws: [sprite({anchor: [1, Infinity]})]}]), /Invalid scene sprite/);
  assert.throws(() => collectTextures([{kind: 'shadow', draws: [{kind: 'ellipse', cx: 0, cy: 0, rx: 1, ry: 1, alpha: Number.NaN}]}]), /Invalid scene ellipse/);
  assert.throws(() => collectTextures([{kind: 'shadow', draws: null}]), /Invalid scene draw list/);
  assert.throws(() => collectTextures([{kind: 'made-up'}]), /Unsupported/);
  assert.deepEqual([...collectTextures([sprite()])], ['a']);
});
function fixture(options = {}) {
  let callbackId = 0; const callbacks = new Map(); global.requestAnimationFrame = fn => {callbacks.set(++callbackId, fn); return callbackId;}; global.cancelAnimationFrame = id => callbacks.delete(id);
  const errors = [], ready = [], renderer = new ArenaSceneRenderer(canvas(), {textureUrl: id => '/textures/' + id + '.png', forceCanvas: true, onError: e => errors.push(e.message), onReady: p => ready.push(p), ...options});
  renderer.painter.upload = image => image;
  const loading = [];
  renderer.loadTexture = (id, signal) => new Promise((resolve, reject) => {signal.addEventListener('abort', () => reject(signal.reason), {once: true}); loading.push({id, signal, resolve: () => resolve({image: {width: 1, height: 1, pixels: new Uint8ClampedArray([255, 0, 0, 255])}, bytes: 80}), reject});});
  return {renderer, loading, errors, ready, frame() {const list = [...callbacks.values()]; callbacks.clear(); for (const fn of list) fn(performance.now());}};
}
const settle = () => new Promise(resolve => setImmediate(resolve));
test('packet queue and concurrent downloads stay bounded while each texture is fetched once', async () => {
  const f = fixture({loadConcurrency: 2});
  for (let i = 1; i <= 25; i++) f.renderer.push(packet(i));
  assert.equal(f.renderer.stats.pending_frames, 4); assert.equal(f.loading.length, 1); assert.equal(f.renderer.stats.dropped, 21); assert.equal(f.ready.length, 0);
  f.loading[0].resolve(); await settle(); f.frame();
  assert.equal(f.ready.length, 1); assert.equal(f.renderer.stats.texture_count, 1); assert.equal(f.renderer.stats.download_bytes, 80);
  f.renderer.push(packet(26)); f.frame(); assert.equal(f.loading.length, 1); assert.equal(f.errors.length, 0);
  f.renderer.destroy(); assert.equal(f.renderer.stats.texture_bytes, 0);
});
test('a decoded pose stays on the GPU when the next match starts', async () => {
  const f = fixture();
  f.renderer.push(packet(1, {match_id: 1}));
  f.loading[0].resolve(); await settle();
  assert.equal(f.renderer.stats.texture_count, 1);
  f.renderer.push(packet(1, {match_id: 2}));
  assert.equal(f.renderer.stats.texture_count, 1);
  assert.equal(f.loading.length, 1);
  f.renderer.destroy();
  assert.equal(f.renderer.stats.texture_count, 0);
});
test('new matches reset the timeline and abort old loads; stale completion cannot publish', async () => {
  const f = fixture(); f.renderer.push(packet()); const old = f.loading[0];
  f.renderer.push(packet(1, {match_id: 2})); assert.equal(old.signal.aborted, true); old.resolve(); await settle();
  assert.equal(f.renderer.stats.texture_count, 0); assert.equal(f.ready.length, 0);
  f.loading[1].resolve(); await settle(); f.frame(); assert.equal(f.ready.length, 1); assert.equal(f.ready[0].match_id, 2);
  f.renderer.destroy();
});
test('incomplete poses never render partial objects and missing textures fail explicitly', async () => {
  const f = fixture(); f.renderer.push(packet()); f.frame(); assert.equal(f.ready.length, 0);
  f.loading[0].reject(new Error('Texture missing')); await settle();
  assert.deepEqual(f.errors, ['Texture missing']); assert.equal(f.renderer.stats.error, 'Texture missing');
  f.renderer.reset(1); assert.equal(f.renderer.halted, false); f.renderer.destroy();
});
test('paused scenes paint the authored frame and do not spin requestAnimationFrame continuously', async () => {
  const f = fixture(); f.renderer.push(packet(1, {status: 'paused'})); f.loading[0].resolve(); await settle(); f.frame();
  const count = f.renderer.stats.frames; f.frame(); assert.equal(f.renderer.stats.frames, count); f.renderer.destroy();
});
test('destroy aborts work and closes all visible cached resources', async () => {
  const f = fixture(); f.renderer.push(packet()); f.renderer.destroy();
  assert.equal(f.loading[0].signal.aborted, true); f.loading[0].resolve(); await settle();
  assert.equal(f.renderer.stats.texture_count, 0); assert.equal(f.ready.length, 0);
});
test('cache eviction protects every displayed/pending pose and releases only older unpinned resources', () => {
  const f = fixture({maxTextures: 32}), released = [];
  f.renderer.painter.release = resource => released.push(resource.name);
  for (let i = 0; i < 40; i++) f.renderer.textures.set(i.toString(16), {resource: {name: i.toString(16)}, bytes: 4, used: i});
  f.renderer.bytes = 160;
  f.renderer.timeline = [{ids: new Set(['0', '1'])}];
  f.renderer.pending = [{ids: new Set(['2', '3'])}];
  assert.equal(f.renderer.trim(), true);
  assert.equal(f.renderer.textures.size, 31);
  for (const id of ['0', '1', '2', '3']) assert.equal(f.renderer.textures.has(id), true);
  assert.deepEqual(released, ['4', '5', '6', '7', '8', '9', 'a', 'b', 'c']);
  assert.equal(f.renderer.stats.evicted_textures, 9); f.renderer.destroy();
});

test('HUD/root draw-order IDs switch discretely when inserted shades renumber unrelated components', () => {
  const older = [sprite({id: 'hud-0-3'}), sprite({id: 'root-0'})];
  const newer = [sprite({id: 'hud-0-3', tex: 'b', m: [1, 0, 0, 1, 200, 100]}), sprite({id: 'root-0', tex: 'c', m: [2, 0, 0, 2, 40, 50]})];
  const middle = interpolateDraws(older, newer, .75);
  assert.equal(middle[0], older[0]); assert.equal(middle[1], older[1]);
  assert.equal(interpolateDraws(older, newer, 1), newer);
  const moving = interpolateDraws([sprite({id: 'entity-1-part-0', m: [1, 0, 0, 1, 0, 0]})], [sprite({id: 'entity-1-part-0', m: [1, 0, 0, 1, 10, 20]})], .5);
  assert.deepEqual(moving[0].m, [1, 0, 0, 1, 5, 10]);
  const hudOlder = [sprite({id: 'hud-1', anchor: [1, 2]})];
  const hudAnchored = interpolateDraws(hudOlder, [sprite({id: 'hud-1', tex: 'b', anchor: [30, 40], m: [1, 0, 0, 1, 30, 40]})], .5);
  assert.equal(hudAnchored[0], hudOlder[0]);
  assert.equal(hudAnchored[0].tex, 'a');
  assert.equal(hudAnchored[0].m[4], 0);
});


test('fast playback retains a full wall-clock interpolation buffer between coalesced poses', () => {
  const f = fixture({interpolationDelay: 50});
  let displayed;
  f.renderer.painter.paint = (_packet, draws) => { displayed = draws; };
  f.renderer.timeline = [
    {packet: packet(1, {tick: 0, draws: [sprite()], playback_speed: 4}), nativeTime: 0, received: 50, ids: new Set()},
    {packet: packet(2, {tick: 4, draws: [sprite({m: [1, 0, 0, 1, 100, 0]})], playback_speed: 4}), nativeTime: 200, received: 100, ids: new Set()}
  ];
  f.renderer.render(125);
  assert.equal(displayed[0].m[4], 50);
  assert.equal(f.errors.length, 0);
  f.renderer.destroy();
});

test('out-of-order texture completion cannot bypass an incomplete pose prefix', async () => {
  const f = fixture({loadConcurrency: 2});
  f.renderer.push(packet(1, {draws: [sprite({tex: 'a'})]}));
  f.renderer.push(packet(2, {draws: [sprite({tex: 'b'})]}));
  f.loading.find(load => load.id === 'b').resolve(); await settle();
  assert.equal(f.renderer.timeline.length, 0); assert.equal(f.renderer.pending.length, 2);
  f.frame(); assert.equal(f.ready.length, 0);
  f.loading.find(load => load.id === 'a').resolve(); await settle();
  assert.deepEqual(f.renderer.timeline.map(frame => frame.packet.sequence), [1, 2]);
  assert.equal(f.renderer.pending.length, 0); f.frame(); assert.equal(f.ready.length, 1);
  f.renderer.destroy();
});
test('a simultaneously decoded spawn/attack burst retains its coherent intermediate poses', async () => {
  const f = fixture({loadConcurrency: 3});
  for (let i = 1; i <= 3; i++) f.renderer.push(packet(i, {draws: [sprite({tex: ['a', 'b', 'c'][i - 1], m: [1, 0, 0, 1, i * 20, 0]})]}));
  for (const load of f.loading) load.resolve(); await settle();
  assert.deepEqual(f.renderer.timeline.map(frame => frame.packet.sequence), [1, 2, 3]);
  assert.equal(f.renderer.timeline[0].packet.draws[0].tex, 'a');
  assert.equal(f.renderer.timeline[1].packet.draws[0].tex, 'b');
  f.renderer.destroy();
});
test('bounded pending overflow eventually sheds a blocked prefix after the scene is visible', async () => {
  const f = fixture({maxPendingFrames: 2});
  f.renderer.push(packet(1)); f.loading[0].resolve(); await settle(); f.frame();
  f.renderer.push(packet(2, {draws: [sprite({tex: 'b'})]}));
  f.renderer.push(packet(3)); assert.equal(f.renderer.timeline.at(-1).packet.sequence, 1);
  f.renderer.push(packet(4));
  assert.equal(f.renderer.pending.length, 0);
  assert.deepEqual(f.renderer.timeline.map(frame => frame.packet.sequence), [1, 3, 4]);
  assert.equal(f.renderer.stats.dropped, 1); f.renderer.destroy();
});

test('HTTP410 produces a typed expired-pose error while HTTP404 remains a missing-asset failure', async () => {
  const f = fixture(), fetch = global.fetch;
  try {
    global.fetch = async () => ({status: 410, ok: false});
    await assert.rejects(ArenaSceneRenderer.prototype.loadTexture.call(f.renderer, 'a', new AbortController().signal), error => error.code === 'ARENA_TEXTURE_EXPIRED' && error.textureId === 'a');
    global.fetch = async () => ({status: 404, ok: false});
    await assert.rejects(ArenaSceneRenderer.prototype.loadTexture.call(f.renderer, 'a', new AbortController().signal), error => !error.code && /404/.test(error.message));
  } finally {global.fetch = fetch; f.renderer.destroy();}
});
test('expired poses drop atomically, preserve the displayed scene, abort obsolete loads and recover on a fresh packet', async () => {
  const f = fixture({maxPendingFrames: 3});
  f.renderer.push(packet(1)); f.loading[0].resolve(); await settle(); f.frame();
  f.renderer.push(packet(2, {draws: [sprite({tex: 'b'})]}));
  f.renderer.push(packet(3, {draws: [sprite({tex: 'b'}), sprite({id: 'troop-13', tex: 'c'})]}));
  const expired = new Error('Original pose expired'); expired.code = 'ARENA_TEXTURE_EXPIRED';
  f.loading.find(load => load.id === 'b').reject(expired); await settle();
  assert.equal(f.renderer.pending.length, 0); assert.deepEqual(f.renderer.timeline.map(frame => frame.packet.sequence), [1]);
  assert.equal(f.loading.find(load => load.id === 'c').signal.aborted, true);
  assert.equal(f.renderer.halted, false); assert.deepEqual(f.errors, []);
  assert.equal(f.renderer.stats.expired_textures, 1); assert.equal(f.renderer.stats.dropped, 2);
  assert.equal(f.renderer.stats.loading_textures, 0);
  f.renderer.push(packet(4, {draws: [sprite({tex: 'b'})]}));
  const renewed = f.loading.filter(load => load.id === 'b'); assert.equal(renewed.length, 2);
  renewed[1].resolve(); await settle(); f.frame();
  assert.equal(f.renderer.timeline.at(-1).packet.sequence, 4); assert.equal(f.renderer.pending.length, 0);
  assert.equal(f.renderer.halted, false); assert.equal(f.ready.length, 1); f.renderer.destroy();
});
test('a genuine missing texture remains fatal rather than disappearing through expired-pose recovery', async () => {
  const f = fixture(); f.renderer.push(packet());
  f.loading[0].reject(new Error('Original texture could not be loaded (404): a')); await settle();
  assert.equal(f.renderer.halted, true); assert.equal(f.renderer.stats.expired_textures, 0);
  assert.equal(f.errors.length, 1); assert.equal(f.ready.length, 0); f.renderer.destroy();
});


test('a cold pose starts its display clock when assets finish so interpolation keeps drawing', async () => {
  const f = fixture();
  f.renderer.push(packet());
  f.renderer.pending[0].received -= 1000;
  f.loading[0].resolve(); await settle();
  f.frame();
  const count = f.renderer.stats.frames;
  f.frame();
  assert.equal(f.renderer.stats.frames, count + 1);
  assert.equal(f.errors.length, 0);
  f.renderer.destroy();
});

test('a paused timeline paints the authored pose instead of the in-between frame', () => {
  const f = fixture();
  let shown;
  f.renderer.painter.paint = (_packet, draws) => {shown = draws;};
  f.renderer.timeline = [
    {packet: packet(1, {draws: [sprite({m: [1, 0, 0, 1, 0, 0]})]}), nativeTime: 0, readyAt: 0, received: 0, ids: new Set()},
    {packet: packet(2, {status: 'paused', draws: [sprite({m: [1, 0, 0, 1, 80, 0]})]}), nativeTime: 50, readyAt: 0, received: 0, ids: new Set()}
  ];
  f.renderer.render(1000);
  assert.equal(shown[0].m[4], 80);
  assert.equal(f.renderer.frameHandle, 0);
  f.renderer.destroy();
});

test('exact fixture display keeps cached textures when the tick and match move backward', async () => {
  const f = fixture();
  let shown;
  f.renderer.painter.paint = (_packet, draws) => {shown = draws;};
  const first = f.renderer.displayExact(packet(4, {tick: 20, match_id: 1, draws: [sprite({tex: 'a', m: [1, 0, 0, 1, 8, 0]})]}));
  f.loading[0].resolve(); await settle(); f.frame();
  const painted = await first;
  assert.equal(painted.exact, true);
  assert.equal(painted.tick, 20);
  assert.equal(shown[0].m[4], 8);
  const resident = f.renderer.textures.get('a');
  const second = f.renderer.displayExact(packet(1, {tick: 2, match_id: 8, draws: [sprite({tex: 'b', m: [1, 0, 0, 1, 1, 0]})]}));
  assert.equal(f.renderer.textures.get('a'), resident);
  f.loading.find(load => load.id === 'b').resolve(); await settle(); f.frame();
  const later = await second;
  assert.equal(later.exact, true);
  assert.equal(later.tick, 2);
  assert.equal(shown[0].tex, 'b');
  assert.equal(shown[0].m[4], 1);
  assert.equal(f.renderer.textures.get('a'), resident);
  assert.equal(f.renderer.textures.has('b'), true);
  assert.equal(f.renderer.diagnostics().exact, true);
  assert.equal(f.renderer.diagnostics().match_id, 8);
  assert.equal(f.loading.length, 2);
  f.renderer.destroy();
});

test('gallery file URLs stay under the deployment prefix and reject traversal', () => {
  assert.equal(galleryFileUrl('/clashroyale', 'cases/pekka/0.json'), '/clashroyale/api/visual-gallery/files/cases/pekka/0.json');
  assert.equal(galleryFileUrl('', 'textures/ab.png'), '/api/visual-gallery/files/textures/ab.png');
  assert.equal(galleryFileUrl('/clashroyale/', 'textures/ab cd.png'), '/clashroyale/api/visual-gallery/files/textures/ab%20cd.png');
  for (const relative of ['../secret', '/etc/passwd', 'a/../../b', 'a//b', './pose.json', 'pose.json?x=1']) {
    assert.throws(() => galleryFileUrl('/arena', relative), /not relative/);
  }
  const html = fs.readFileSync(path.join(__dirname, 'static/gallery.html'), 'utf8');
  assert.match(html, /\/api\/visual-gallery\/manifest/);
  assert.match(html, /arenaGalleryFileUrl/);
  assert.match(html, /displayExact/);
  assert.match(html, /preserveDrawingBuffer:\s*true/);
  assert.match(html, /CPU reference parity is not official game animation parity/);
  assert.match(html, /galleryDiagnostics/);
  assert.match(html, /galleryHelper/);
  assert.doesNotMatch(html, /href="\/play\.html"/);
  assert.doesNotMatch(html, /passed frames/i);
});
