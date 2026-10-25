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
  const progressExample = render('enrichmentPanel', {
    id:'replay-enrichment-example', kind:'data enrichment', status:'running', host:'home',
    validation:{status:'unverified'}, metric_source:'state.json', progress:{
      total:64275, done:13665, candidate:50641, attempted:31, preflight_skipped:13634,
      simulated_unverified:0, partial_unverified:27, failed:4, enriched:0,
      state_rows_exported:89, files_per_second:.1753, eta_seconds:288660, workers:2,
    },
  });
  assert.ok(progressExample.includes('Harness attempts'));
  assert.ok(progressExample.includes('31 / 50,641'));
  assert.ok(progressExample.includes('13,634 skipped'));
  assert.ok(progressExample.includes('50,610 remaining'));
  assert.ok(progressExample.includes('aria-valuenow="0.061"'));
  assert.ok(!progressExample.includes('13,665 / 64,275'));
  console.log('Harness progress excludes instant skips OK');
  const imitationExample = render('runDetailHTML', {summary:{
    id:'banana-2',kind:'training run',status:'running',reported_status:'running',
    phase:'Epoch 1/3',training_method:'imitation_native_aux',metric_source:'state.json',
    progress:{done:90,total:17901,eta_seconds:2100},
    training_data:{train_battles:45826,train_action_rows:1145610,train_rows:19078,validation_rows:1122},
    report:'/reports/banana2.html',evidence:[],counters:{},
  },logs:[],weights:[],artifacts:[],files:[]});
  assert.ok(imitationExample.includes('90 / 17,901'));
  assert.ok(imitationExample.includes('Native auxiliary rows'));
  assert.ok(imitationExample.includes('/reports/banana2.html'));
  assert.ok(!imitationExample.includes('Wins / losses'));
  console.log('Imitation run progress uses actions and auxiliary rows OK');
  const tournamentExample = render('runDetailHTML', {summary:{
    id:'banana-battle-royal-eval-example',kind:'eval',status:'running',reported_status:'running',
    phase:'Frozen Banana round robin',training_method:'banana_round_robin',metric_source:'state.json',
    progress:{done:12,total:144,eta_seconds:1000},evidence:[],counters:{},
    tournament:{valid_games:11,voids:1,standings:[{model:'banana-2',wins:5,losses:2,draws:1,voids:1,score:.6875}],pairings:{}},
  },logs:[],weights:[],artifacts:[],files:[]});
  assert.ok(tournamentExample.includes('12 / 144'));
  assert.ok(tournamentExample.includes('Standings'));
  assert.ok(tournamentExample.includes('Void matches'));
  assert.ok(tournamentExample.includes('banana-2'));
  console.log('Tournament standings and void exclusions OK');
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
