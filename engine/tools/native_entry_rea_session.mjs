// One persistent REA session. MCP SDK options belong in callTool's second argument.
import {Client} from './rea/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import {StdioClientTransport} from './rea/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import {createInterface} from 'node:readline';
import {mkdir, writeFile} from 'node:fs/promises';
import path from 'node:path';

const root=path.resolve(import.meta.dirname,'..');
const directory=path.resolve(root,process.argv[2]??'analysis/native-entry/rea-decrypted');
await mkdir(directory,{recursive:true});
const env={...process.env,GHIDRA_INSTALL_DIR:path.join(root,'tools/ghidra_12.1.4_PUBLIC'),REA_GHIDRA_STARTUP_TIMEOUT_MS:'900000',GHIDRA_HEADLESS_MAXMEM:'4G',REA_JADX_MCP_JAR:path.join(root,'tools/jadx-headless-mcp-0.7.1-all.jar')};
const transport=new StdioClientTransport({command:process.execPath,args:[path.join(root,'tools/rea/node_modules/rea-agents/scripts/rea.mjs'),'--mcp'],env,cwd:root,stderr:'inherit',maxBufferSize:64*1024*1024});
const client=new Client({name:'clash-native-entry-investigation',version:'1'},{capabilities:{}});
await client.connect(transport);
const metadata=await client.listTools();
await writeFile(path.join(directory,'tools.json'),JSON.stringify(metadata,null,2));
process.stdout.write(JSON.stringify({ready:true})+'\n');
try{
  for await(const line of createInterface({input:process.stdin,crlfDelay:Infinity})){
    if(!line.trim())continue;
    try{
      const request=JSON.parse(line);
      if(!metadata.tools.some(t=>t.name===request.name))throw Error('Tool is not advertised');
      const result=await client.callTool({name:request.name,arguments:request.arguments??{}},{timeout:900000});
      const file=path.join(directory,path.basename(request.save??`${Date.now()}-${request.name}.json`));
      await writeFile(file,JSON.stringify(result,null,2));
      const evidence=result.structuredContent;
      process.stdout.write(JSON.stringify({tool:request.name,file,isError:result.isError??false,evidence_id:evidence?.evidence_id,preview:String(evidence?.normalized_result??JSON.stringify(evidence)).slice(0,240)})+'\n');
    }catch(error){process.stdout.write(JSON.stringify({error:error.message})+'\n');}
  }
}finally{await client.close();}
