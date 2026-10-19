// ARM64 NativeBridge realm: observe real guest-library initialization and calls.
const emit=(event,data={})=>send({event,arch:Process.arch,pid:Process.id,...data});
let companion=null,libc=null;
const hooked=new Set();
function hook(address,label,callbacks){
  const key=String(address);if(hooked.has(key))return;
  try{Interceptor.attach(address,callbacks);hooked.add(key);emit('hook-installed',{label,address:key});}
  catch(error){emit('hook-failed',{label,address:key,error:String(error)});}
}
function callbacks(){
  if(!companion)return;
  const slots={};for(const rva of [0x6c6468,0x6c6470]){try{const address=companion.base.add(rva).readPointer();slots['0x'+rva.toString(16)]={address:String(address),rva:String(address.sub(companion.base)),bytes:hexdump(address,{length:24,header:false,ansi:false})};}catch(error){slots[String(rva)]=String(error);}}
  emit('callback-slots',slots);
}
Process.attachModuleObserver({onAdded(module){
  emit('module-added',{name:module.name,base:String(module.base),size:module.size,path:module.path});
  if(module.name==='libc.so'){
    libc=module;
    for(const name of ['abort','exit','_exit','kill','tgkill','pthread_kill']){
      try{hook(module.getExportByName(name),'libc.'+name,{onEnter(args){emit('termination-call',{name,arguments:[String(args[0]),String(args[1])],returnAddress:String(this.returnAddress),backtrace:Thread.backtrace(this.context,Backtracer.ACCURATE).map(DebugSymbol.fromAddress).map(String)});callbacks();}});}catch(error){emit('missing-export',{name,error:String(error)});}
    }
  }
  if(module.name==='libsupercell_clashroyale.so'){
    companion=module;
    for(const rva of [0x65ccf0,0x68179c,0x681780]){
      hook(module.base.add(rva),'companion+'+rva.toString(16),{onEnter(args){emit('companion-enter',{rva:'0x'+rva.toString(16),arguments:[String(args[0]),String(args[1]),String(args[2])]});},onLeave(result){emit('companion-return',{rva:'0x'+rva.toString(16),result:String(result)});callbacks();}});
    }
  }
}});
emit('arm-trace-ready');
