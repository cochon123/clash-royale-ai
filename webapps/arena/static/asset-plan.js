/* Which immutable pack files a deck pair needs, and how their URLs resolve. */
(function (root) {
  "use strict";
  function directoryUrl(manifestUrl) {
    const url = new URL(manifestUrl, "http://assets.local/");
    url.pathname = url.pathname.replace(/[^/]*$/, "");
    url.search = "";
    url.hash = "";
    return url;
  }
  function resolveAssetUrl(manifestUrl, relative) {
    if (typeof relative !== "string" || relative === "" || relative.startsWith("/") || relative.includes("\\") || relative.includes("..") || relative.includes("?") || relative.includes("#")) {
      throw new Error("asset path is not relative");
    }
    return new URL(relative, directoryUrl(manifestUrl)).href;
  }
  function planPackFiles(manifest, nativeRows) {
    if (!manifest || manifest.v !== 1 || !manifest.files || typeof manifest.files !== "object") throw new Error("asset manifest is invalid");
    const paths = [];
    const seen = new Set();
    const add = (path) => {
      if (typeof path !== "string" || seen.has(path)) return;
      seen.add(path);
      paths.push(path);
    };
    for (const path of manifest.always || []) add(path);
    for (const row of nativeRows || []) for (const path of (manifest.cards && manifest.cards[row]) || []) add(path);
    const files = [];
    let bytes = 0;
    for (const path of paths) {
      const meta = manifest.files[path];
      if (!meta || typeof meta.url !== "string") throw new Error("manifest is missing " + path);
      const size = Number(meta.bytes) || 0;
      bytes += size;
      files.push({path, url: meta.url, bytes: size, sha256: meta.sha256 || ""});
    }
    let wasm = null;
    if (typeof manifest.painter_wasm === "string" && manifest.painter_wasm) {
      const listed = Object.values(manifest.files).find((meta) => meta && meta.url === manifest.painter_wasm);
      const size = listed ? Number(listed.bytes) || 0 : 0;
      wasm = {path: manifest.painter_wasm, url: manifest.painter_wasm, bytes: size};
      bytes += size;
    }
    return {files, wasm, bytes};
  }
  root.planPackFiles = planPackFiles;
  root.resolveAssetUrl = resolveAssetUrl;
  if (typeof module !== "undefined" && module.exports) module.exports = {planPackFiles, resolveAssetUrl};
})(typeof globalThis !== "undefined" ? globalThis : this);
