frontend: frontend_build rollup_js

frontend_build:
	cd frontend \
	&& wasm-pack build --target web  --out-dir ../static/wasm-pack

rollup_js:
	cd static \
	&& rollup ./main.js --format iife --file ./wasm-pack/bundle.js

backend:
	cd backend \
	&& cargo build

run: backend frontend
	cd backend \
	&& cargo run
