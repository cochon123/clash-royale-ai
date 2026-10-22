// Passive map/memory capture of the supplied APK in the disposable offline AVD.
// ARM64 modules loaded by NativeBridge may not appear in enumerateModules().
const prefix = '/data/user/0/com.supercell.clashroyale/cache/clash-re-';
let mapsLast = '';
const seen = new Set();
let count = 0;
function capture(reason) {
  try {
    const maps = File.readAllText('/proc/self/maps');
    if (maps !== mapsLast) {
      File.writeAllText(prefix + 'maps.txt', maps);
      mapsLast = maps;
    }
    for (const line of maps.split('\n')) {
      if (!line.includes('libg.so') && !line.includes('libsupercell_clashroyale.so') &&
          !line.includes('/arm64/libc.so')) continue;
      const fields = line.trim().split(/\s+/);
      const [start, end] = fields[0].split('-');
      const key = fields[0] + '-' + fields[2];
      if (!fields[1].includes('r') || seen.has(key)) continue;
      const address = ptr('0x' + start);
      const size = ptr('0x' + end).sub(address).toUInt32();
      if (size > 128 * 1024 * 1024) continue;
      const name = fields.at(-1).split('/').at(-1);
      const filename = prefix + name + '-' + fields[0] + '-' + fields[2] + '.bin';
      const file = new File(filename, 'wb');
      for (let offset = 0; offset < size; offset += 1024 * 1024) {
        file.write(address.add(offset).readByteArray(Math.min(1024 * 1024, size - offset)));
      }
      file.close();
      seen.add(key);
      send({ event: 'memory-range-saved', line, filename, reason });
    }
    count++;
  } catch (error) { send({ event: 'capture-error', error: String(error), reason }); }
}
setInterval(() => capture('poll'), 20);
send({ event: 'map-probe-ready', arch: Process.arch });
Java.performNow(function () {
  const Handler = Java.use('com.android.internal.os.RuntimeInit$KillApplicationHandler');
  Handler.uncaughtException.implementation = function (thread, exception) {
    capture('uncaught-exception');
    send({ event: 'startup-exception', exception: String(exception) });
    return this.uncaughtException(thread, exception);
  };
});
