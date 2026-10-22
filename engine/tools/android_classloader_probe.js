// Inspect the original application's classloader before Application.attach.
Java.performNow(function () {
  const Runtime = Java.use('java.lang.Runtime');
  const methods = Runtime.class.getDeclaredMethods();
  send({ event: 'runtime-load-methods', methods: Array.from(methods).map(String).filter(x => /[lL]oad/.test(x)) });
  const Thread = Java.use('android.app.ActivityThread');
  const current = Thread.currentActivityThread();
  send({ event: 'activity-thread', present: current !== null });
  if (current !== null) {
    const bound = current.mBoundApplication.value;
    send({ event: 'bound-application', present: bound !== null });
    if (bound !== null) {
      const loader = bound.info.value.getClassLoader();
      send({ event: 'app-classloader', loader: String(loader) });
    }
  }
});
