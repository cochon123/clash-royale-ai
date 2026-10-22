# Local browser scene prototype

The authoritative engine and AI stay server-side. Rust reuses the original
Painter's animation selection, art, z order and layout to produce a display
list. The browser composites cached original sprites and interpolates their
transforms at display rate. This first step retains presentation planning on
the server; it does not claim the complete renderer runs client-side.

## Native presenter

`clash-web-scene --assets DIR --texture-cache DIR` reads the same `Snapshot`
JSON lines as `clash-web-renderer`. Every snapshot is observed so transient
spawn, impact and death events are retained. Display lists are paced by wall
clock (`--fps 30` by default); the latest pending pose is emitted even if input
becomes idle. One output line has this shape:

```json
{"v":1,"sequence":1,"tick":100,"display_tick":100.0,"match_id":1,
 "width":540,"height":900,"clear":[19,27,31,255],
 "layout":{"field":[43,125,454,648],"world":[18000,32000],"cards":[]},
 "textures":[{"id":"content-hash","width":64,"height":64}],
 "draws":[{"id":"entity-12-part-0","kind":"sprite","tex":"content-hash",
           "m":[1,0,0,1,100,200],"alpha":1,"blend":"normal","add":[0,0,0]}]}
```

- `m` is the Canvas affine ordering `[a,b,c,d,tx,ty]`, mapping
  `(x,y)` to `(a*x+c*y+tx,b*x+d*y+ty)` in 540 by 900 pixels.
- `sprite`: `tex`, `m`, `alpha` (0..1), `blend` (`normal`, `add`,
  `premultiplied-add`, `premultiplied-screen`), `add` (RGB byte offsets).
- `rect`: `x,y,w,h,color` (RGBA bytes), for simple authored HUD geometry.
- Sprite/rect `replace:true` means literal pixel replacement, including alpha,
  rather than alpha compositing (original imageops::replace/fill_rect semantics).
- `shadow`: `draws` containing sprite/ellipse silhouettes. Their alpha masks
  combine by maximum, then darken toward `[0,0,100]` once, matching ShadowMask.
- Nested shadow sprite transforms use the same ordering. `ellipse` contains
  `cx,cy,rx,ry,alpha`; it uses the existing soft-blob falloff.
- Draw IDs must be stable for the same entity/part across texture/pose changes
  and layering changes. Client interpolation must never join different matches.
- `textures` lists newly cached textures, not inline image data. Files are
  immutable `DIR/<id>.png`, with IDs restricted to lowercase hex. Native cache
  eviction preserves current/recent poses; an expired owner-scoped URL returns
  410 with no-store, prompting the client to discard that stale pending pose. Dynamic
  positions must not be baked into textures. Avoid full-frame residual images.
- No original engine addresses, filesystem paths, hidden opponent hands or
  hidden policy plans should be exposed. Draw commands already reflect the
  original hidden-hand spectator view.
- Static original art and cached animation poses can be rasterized once;
  composite full-frame pixels and JPEG are excluded from the primary path.

## Browser and HTTP integration

The parent integrates `GET /api/matches/{id}/scene` (SSE `scene` events) and
`GET /api/matches/{id}/textures/{texture}.png`, with existing owner checks.
All paths retain an optional deployment base prefix. The browser receives the
regular compact game-state SSE separately. Both SSE paths negotiate gzip with
a sync flush per event, preserving immediate delivery. Native display-list output
and state delivery are coalesced to at most 30 updates/s during fast playback;
this does not cap client requestAnimationFrame or change combat timing. JPEG `/view` and `/video` are lazy,
explicit fallback paths, never fetched automatically in browser-scene mode.

Browser module `scene-renderer.js` exports `window.ArenaSceneRenderer`:

```js
const view = new ArenaSceneRenderer(canvas, {
  textureUrl: id => `${BASE}/api/matches/${matchId}/textures/${id}.png`,
  onReady: packet => {}, onError: error => {}
});
view.push(packet); // packet from SSE, loads textures and queues bounded poses
view.destroy();
view.stats; // frame rate, texture bytes/count, queue length, missing textures
```

Prefer WebGL2 and support a Canvas2D fallback. Fetch each texture once per view,
bound pending packets and texture memory, preserve draw order/blend modes,
interpolate stable transforms with approximately 50ms of wall-clock buffering
from complete-pose readiness,
freeze on pause/ended (packets can carry `status`), and recover on reset.
The parent renders local selection/aim UI above the scene, using its layout.

## Input integration

`input-transport.js` exports `window.ArenaInputTransport`. Constructor accepts
`{send: async command => ..., onError: error => ...}`. Public API is
`enqueue(command)`, `reset(matchId)`, `destroy()`, and `stats`.
Only one replaceable aim may wait; play/cancel clear pending obsolete aims
and take priority. Never queue an unbounded mouse history or run stale commands
after a reset. Commands already carry match_id, serial, slot and card_id.
Server `play` atomically selects the supplied currently available card before
validation; it must reject stale match/card/serial/paused/ended commands.
