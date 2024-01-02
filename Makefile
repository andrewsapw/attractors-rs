serve:
	python3 -m http.server 8080

build-web:
	ls src/*.rs | entr cargo build --target wasm32-unknown-unknown --release

build: 
	cargo build --release