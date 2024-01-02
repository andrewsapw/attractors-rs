serve:
	python3 -m http.server 8080

.PHONY: help
help: # Show help for each of the Makefile recipes.
	@grep -E '^[a-zA-Z0-9 -]+:.*#'  Makefile | sort | while read -r l; do printf "\033[1;32m$$(echo $$l | cut -f 1 -d':')\033[00m:$$(echo $$l | cut -f 2- -d'#')\n"; done

.PHONY: build-web
build-web: # Live reload build for web
	ls src/*.rs | entr cargo build --target wasm32-unknown-unknown --release

.PHONY: build
build: # Regular build
	cargo build --release