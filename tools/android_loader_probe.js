// Active, offline startup investigation for the disposable project AVD.
// Original APK bytes remain unchanged. All interventions are emitted as events.
const log = (event, values={}) => send({event,...values});
const maps = () => File.readAllText('/proc/self/maps');
let appLoader=null,appCaller=null,attempted=false;
function forceGameLoad(){
  if(attempted||appLoader===null)return;attempted=true;
  try{const Runtime=Java.use('java.lang.Runtime');const filename=appLoader.findLibrary('g');log('forced-game-load-enter',{filename:String(filename)});const error=Runtime.nativeLoad.overload('java.lang.String','java.lang.ClassLoader','java.lang.Class').call(Runtime,filename,appLoader,appCaller);log('forced-game-load-return',{error:String(error)});snapshot('forced-game-load-return');}catch(error){log('forced-game-load-error',{error:String(error)});}
}
function snapshot(reason) {
  try {
    const text=maps();
    File.writeAllText('/data/user/0/com.supercell.clashroyale/cache/loader-probe-maps.txt',text);
    log('map-summary',{reason,pid:Process.id,lines:text.split('\n').filter(x=>/libg\.so|libsupercell_clashroyale/.test(x))});
  }catch(error){log('map-error',{error:String(error)});}
}
Java.performNow(function(){
  const Handler=Java.use('com.android.internal.os.RuntimeInit$KillApplicationHandler');
  Handler.uncaughtException.implementation=function(thread,error){log('suppressed-java-termination',{exception:String(error)});snapshot('java-termination');forceGameLoad();};
  const Runtime=Java.use('java.lang.Runtime');
  for(const types of [['java.lang.Class','java.lang.String'],['java.lang.ClassLoader','java.lang.String'],['java.lang.ClassLoader','java.lang.Class','java.lang.String']]){
    try{
      const method=Runtime.loadLibrary0.overload(...types);
      method.implementation=function(...args){
        const name=String(args.at(-1));log('load-enter',{name,signature:types});
        if(/supercell_clashroyale/.test(name)){const candidate=types[0]==='java.lang.Class'?args[0].getClassLoader():args[0];if(candidate!==null)appLoader=Java.retain(candidate);if(types[0]==='java.lang.Class')appCaller=Java.retain(args[0]);else if(types.length===3&&args[1]!==null)appCaller=Java.retain(args[1]);log('saved-app-loader',{loader:String(appLoader),caller:String(appCaller)});}
        const result=method.call(this,...args);log('load-return',{name});snapshot('load-return:'+name);
        const loader=types[0]==='java.lang.Class'?args[0].getClassLoader():args[0];
        if(loader!==null && /supercell_clashroyale/.test(name)){
          log('app-loader-observed',{loader:String(loader)});
          try {
            const factory=Java.ClassFactory.get(loader);
            const W=factory.use('lskumcyej.W');
            W.b.overload('java.lang.String').implementation=function(status){log('suppressed-native-status-dispatch',{encoded:String(status)});snapshot('status-dispatch');};
            log('protection-dispatch-hook-ready');
          }catch(error){log('dispatch-hook-error',{error:String(error)});}
        }
        return result;
      };
    }catch(error){log('hook-signature-error',{types,error:String(error)});}
  }
  log('loader-probe-ready',{arch:Process.arch});
});
setInterval(()=>{try{if(maps().includes('/libg.so'))snapshot('libg-observed');}catch{}},200);
