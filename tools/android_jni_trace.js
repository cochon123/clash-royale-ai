// Passive original JNI startup trace. No return values or arguments are changed.
const log = (event, fields = {}) => send({ event, ...fields });
const active = new Set();
const attached = new Set();
function hook(address, callbacks, name) {
  const key = address.toString();
  if (attached.has(key)) return;
  try {
    Interceptor.attach(address, callbacks);
    attached.add(key);
    log('hook-installed', { name, address: key });
  } catch (error) {
    log('hook-error', { name, address: key, error: String(error) });
  }
}
function installEnvHooks(env) {
  const table = env.readPointer();
  hook(table.add(6 * Process.pointerSize).readPointer(), {
    onEnter(args) {
      this.capture = active.has(Process.getCurrentThreadId());
      if (this.capture) {
        this.name = args[1].readCString();
        log('FindClass-enter', { name: this.name });
      }
    },
    onLeave(result) {
      if (this.capture) log('FindClass-return', { name: this.name, result: String(result) });
    }
  }, 'JNIEnv.FindClass');
  hook(table.add(215 * Process.pointerSize).readPointer(), {
    onEnter(args) {
      this.capture = active.has(Process.getCurrentThreadId());
      if (!this.capture) return;
      const count = args[3].toInt32();
      const methods = [];
      if (count >= 0 && count < 1000) {
        for (let index = 0; index < count; index++) {
          const entry = args[2].add(index * 3 * Process.pointerSize);
          methods.push({ name: entry.readPointer().readCString(),
            signature: entry.add(Process.pointerSize).readPointer().readCString(),
            address: String(entry.add(2 * Process.pointerSize).readPointer()) });
        }
      }
      log('RegisterNatives-enter', { count, methods });
    },
    onLeave(result) {
      if (this.capture) log('RegisterNatives-return', { result: result.toInt32() });
    }
  }, 'JNIEnv.RegisterNatives');
}
Process.attachModuleObserver({
  onAdded(module) {
    if (module.name === 'libg.so') log('original-libg-mapped', { base: String(module.base), size: module.size });
    if (module.name !== 'libsupercell_clashroyale.so') return;
    log('companion-mapped', { base: String(module.base), size: module.size, path: module.path });
    try {
      const address = module.getExportByName('JNI_OnLoad');
      hook(address, {
        onEnter(args) {
          active.add(Process.getCurrentThreadId());
          const vm = args[0];
          const getEnv = vm.readPointer().add(6 * Process.pointerSize).readPointer();
          log('JNI_OnLoad-enter', { vm: String(vm), reserved: String(args[1]), getEnv: String(getEnv),
            callback_6c6468: String(module.base.add(0x6c6468).readPointer()),
            callback_6c6470: String(module.base.add(0x6c6470).readPointer()) });
          hook(getEnv, {
            onEnter(getArgs) {
              this.capture = active.has(Process.getCurrentThreadId());
              if (!this.capture) return;
              this.destination = getArgs[1];
              this.version = getArgs[2].toUInt32();
              log('GetEnv-enter', { version: '0x' + this.version.toString(16) });
            },
            onLeave(result) {
              if (!this.capture) return;
              const code = result.toInt32();
              log('GetEnv-return', { version: '0x' + this.version.toString(16), result: code,
                env: code === 0 ? String(this.destination.readPointer()) : null });
              if (code === 0) installEnvHooks(this.destination.readPointer());
            }
          }, 'JavaVM.GetEnv');
        },
        onLeave(result) {
          log('JNI_OnLoad-return', { result: result.toInt32() });
          active.delete(Process.getCurrentThreadId());
        }
      }, 'companion.JNI_OnLoad');
    } catch (error) { log('module-observer-error', { error: String(error) }); }
  }
});
log('trace-ready', { arch: Process.arch, pid: Process.id });
