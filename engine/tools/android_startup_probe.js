// Research probe: original APK bytes stay unchanged. Suppressing this Java error
// dispatcher only reveals the next startup stage; it does not establish that the
// original battle engine works or that integrity checks have been understood.
Java.performNow(function () {
  const Handler = Java.use('com.android.internal.os.RuntimeInit$KillApplicationHandler');
  Handler.uncaughtException.implementation = function (thread, exception) {
    send({ event: 'uncaught-exception-suppressed', exception: String(exception) });
  };
  send({ event: 'early-system-handler-hook-ready' });
});
Java.perform(function () {
  const W = Java.use('lskumcyej.W');
  W.b.overload('java.lang.String').implementation = function (status) {
    send({ event: 'startup-error-dispatch-suppressed', encoded_status: String(status) });
  };
  send({ event: 'probe-ready', scope: 'lskumcyej.W.b(java.lang.String)' });
});
