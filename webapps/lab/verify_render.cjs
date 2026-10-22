'use strict';
// Renderer and chart-event smoke test. No browser, DOM library, or package install.
const fs = require('node:fs');
const vm = require('node:vm');
const assert = require('node:assert/strict');
const base = process.argv[2] || 'http://127.0.0.1:8801';
const program = fs.readFileSync(__dirname + '/static/app.js','utf8');
const context = vm.createContext({console,URL,URLSearchParams,Intl,Date,Number,Math,Set,JSON,Array,String,
  location:{hostname:'192.168.1.20',pathname:'/',hash:''},AbortSignal});
vm.runInContext(program.slice(0,program.indexOf("$('#arena-link').href=")),context);
async function get(path) {
  const r=await fetch(base+path);
  assert.equal(r.status,200,path);
  return r.json();
}
function render(name, ...args) {
  context.renderArgs=args;
  const result=vm.runInContext(`${name}(...renderArgs)`,context);
  assert.equal(typeof result,'string',name);
  assert.ok(result.length>50,name);
  assert.ok(!result.includes('NaN'),name+' contains NaN');
  return result;
}
class Element {
  constructor(tag='div') {this.tag=tag;this.children=[];this.attrs={};this.style={setProperty(){}};this.classList={toggle(){}};this.offsetWidth=250;this.offsetHeight=100;this.hidden=true;}
  append(n){this.children.push(n);}
  setAttribute(k,v){this.attrs[k]=v;}
  set innerHTML(value){
    this.html=value;this.children=[];this.parts={};
    for (const name of ['legend','plot','crosshair']) if (value.includes(name)) this.parts['.'+name]=new Element(name);
    if(value.includes('<svg'))this.parts.svg=new Element('svg');
    if(this.parts.svg)this.parts.svg.parts={'.crosshair':new Element('line')};
  }
  get innerHTML(){return this.html;}
  querySelector(q){return this.parts?.[q] || null;}
  getBoundingClientRect(){return {left:0,top:0,width:660,height:260};}
}
(async()=>{
  const [overview,models,runs,engine,data,docs]=await Promise.all(['/api/overview','/api/models','/api/runs','/api/engine','/api/data','/api/docs'].map(get));
  for (const [name,value] of [['overviewHTML',overview],['modelsHTML',models],['runsHTML',runs],['engineHTML',engine],['dataHTML',data],['docsHTML',docs]]) {
    const output=render(name,value);
    assert.ok(output.includes('source') || name==='docsHTML',name+' source provenance');
    console.log(name,'OK',output.length,'characters');
  }
  for (const m of models.playable) render('modelDetail',m,'playable');
  for (const m of models.offline) render('modelDetail',m,'offline');
  console.log('All model detail renderers OK');
  let charts=0;
  const tooltip=new Element();
  context.document={createElement:tag=>new Element(tag),querySelector:q=>q==='#tooltip'?tooltip:null};
  context.window={innerWidth:390,innerHeight:844};
  for (const run of runs.runs) {
    const detail=await get('/api/runs/'+encodeURIComponent(run.id));
    render('runDetailHTML',detail);
    // Exercise actual chart construction, legend toggles and pointer tooltip events.
    for (const curve of detail.charts) {
      context.chartRoot=new Element();context.curve=curve;
      vm.runInContext("chart(chartRoot,curve.name,[curve],curve.explanation)",context);
      const panel=context.chartRoot.children[0];
      const button=panel.querySelector('.legend').children[0];
      const plot=panel.querySelector('.plot');
      const svg=plot.querySelector('svg');
      assert.ok(svg,curve.name);
      svg.onpointermove({clientX:180,clientY:160});
      assert.equal(tooltip.hidden,false);
      assert.ok(tooltip.textContent.includes(curve.source));
      svg.onpointerleave();assert.equal(tooltip.hidden,true);
      button.onclick();assert.equal(button.attrs['aria-pressed'],'false');
      button.onclick();assert.equal(button.attrs['aria-pressed'],'true');
      charts++;
    }
  }
  console.log(runs.runs.length,'run details and',charts,'chart renderers/events OK');
  for(const doc of docs.documents) {
    const result=await get('/api/document?path='+encodeURIComponent(doc.path));
    assert.ok(typeof result.html==='string');
    assert.ok(!/<(?:script|iframe|img)\b/i.test(result.html),doc.path);
  }
  console.log(docs.documents.length,'document renders OK');
  context.testURL='http://127.0.0.1:8783/';
  assert.equal(vm.runInContext('localService(testURL)',context),'http://192.168.1.20:8783/');
  console.log('LAN service links OK');
  for(const card of Object.values(engine.cards)) {
    const r=await fetch(base+card.art);assert.equal(r.status,200,card.art);await r.arrayBuffer();
    // Check just the catalog used by the six decks, not unrelated art files.
    if(Object.keys(engine.cards).indexOf(card.id)>10)break;
  }
  console.log('Local card art OK');
})().catch(error=>{console.error(error);process.exitCode=1;});
