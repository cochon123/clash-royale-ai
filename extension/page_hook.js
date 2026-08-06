(() => {
  if (window.__crReplayHookInstalled) return;
  window.__crReplayHookInstalled = true;

  const isReplay = (url) =>
    typeof url === "string" && url.includes("royaleapi.com/data/replay?");

  const publish = (url, text) => {
    let payload;
    try {
      payload = JSON.parse(text);
    } catch {
      return;
    }
    window.postMessage(
      {
        source: "cr-replay-collector",
        type: "replay-payload",
        requestUrl: url,
        referrerUrl: location.href,
        payload
      },
      location.origin
    );
  };

  const originalFetch = window.fetch;
  window.fetch = async function (...args) {
    const response = await originalFetch.apply(this, args);
    const requestUrl =
      typeof args[0] === "string" ? args[0] : args[0] && args[0].url;
    const absolute = requestUrl ? new URL(requestUrl, location.href).href : "";
    if (isReplay(absolute)) {
      response
        .clone()
        .text()
        .then((text) => publish(absolute, text))
        .catch(() => {});
    }
    return response;
  };

  const originalOpen = XMLHttpRequest.prototype.open;
  XMLHttpRequest.prototype.open = function (method, url, ...rest) {
    const absolute = new URL(String(url), location.href).href;
    this.__crReplayUrl = absolute;
    return originalOpen.call(this, method, url, ...rest);
  };

  const originalSend = XMLHttpRequest.prototype.send;
  XMLHttpRequest.prototype.send = function (...args) {
    if (isReplay(this.__crReplayUrl)) {
      this.addEventListener(
        "load",
        () => {
          if (typeof this.responseText === "string") {
            publish(this.__crReplayUrl, this.responseText);
          }
        },
        { once: true }
      );
    }
    return originalSend.apply(this, args);
  };
})();

