.PHONY: clean
OBJS= src/lib.rs

build: $(OBJS)
	cargo build --target wasm32-wasi --release
	cp -f target/wasm32-wasi/release/show_subtitles.wasm target/wasm32-wasi/release/show-subtitles.kpe

clean:
	rm -rf target