'use strict';
// Explicit generation from the built package; never rewrites fixtures in tests.
const fs=require('node:fs'),path=require('node:path');
const root=path.resolve(__dirname,'..'),sdk=require(path.join(root,'bindings/node'));
let count=0;
for(const filename of fs.readdirSync(path.join(root,'examples')).filter(n=>n.endsWith('-result.json'))){
  const input=path.join(root,'examples',filename.replace('-result.json','-request.json'));
  if(!fs.existsSync(input))throw Error('Missing request for '+filename);
  const raw=sdk.calculateJson(fs.readFileSync(input,'utf8')),result=JSON.parse(raw);
  if(result.errors.length)throw Error(filename+': '+JSON.stringify(result.errors));
  if(result.engineVersion!==require(path.join(root,'bindings/node/package.json')).version)throw Error('Stale binary');
  fs.writeFileSync(path.join(root,'examples',filename),raw+'\n');count++;
}
console.log('Regenerated '+count+' example results from the current native package.');
