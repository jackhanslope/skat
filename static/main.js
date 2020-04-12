import init, { run_app } from './wasm-pack/frontend.js';
async function main() {
   await init('/static/wasm-pack/frontend_bg.wasm');
   run_app();
}
main()
