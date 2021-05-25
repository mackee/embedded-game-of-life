const wasm = import('../pkg/index.js');
wasm.then(mod => {
  const l = new mod.LifeOfGame();
  setInterval(() => l.tick(), 50);
});
