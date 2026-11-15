type Record={id:string; response:number};
const values:number[]=[0.98,1.03,1.01,1.07,1.10,1.12,1.15,1.17,1.18,1.19,1.20,1.18,1.22,1.17,1.21,1.19];
const rows:Record[]=values.map((response,i)=>({id:`OCW-B16-${String(i+1).padStart(3,'0')}`,response}));
if(rows.length!==16||rows.some(r=>!Number.isFinite(r.response))) throw new Error('synthetic validation failed');
console.log(`PASS synthetic Study Beta B-16 mean=${(values.reduce((a,b)=>a+b,0)/values.length).toFixed(3)}`);
