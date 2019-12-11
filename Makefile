run:
	time cargo run tests/data/ > salida.json
linux:
	cargo build --release
win32:
	cargo build --release --target=i686-pc-windows-gnu
build: linux win32
	mkdir -p dist
	cp target/i686-pc-windows-gnu/release/hulc2envolventecte.exe dist/
	cp target/release/hulc2envolventecte dist/
	strip dist/hulc2envolventecte.exe
	strip dist/hulc2envolventecte
bloat:
	cargo bloat --release -n 10
	cargo bloat --release --crates -n 10
	# cargo bloat --release --crates --split-std -n 10
perf:
	perf record -g target/release/hulc2envolventecte tests/data
	perf report -f --sort comm,dso
count:
	cargo count -a src/
outdated:
	cargo outdated -R
fixcross:
	$(info [INFO]: Reparando compilación cruzada desde linux a i686-pc-windows-gnu)
	sudo aptitude install -y mingw-w64 mingw-w64-tools
	cp /usr/i686-w64-mingw32/lib/crt2.o ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/i686-pc-windows-gnu/lib/
	cp /usr/i686-w64-mingw32/lib/dllcrt2.o ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/i686-pc-windows-gnu/lib/
	cp /usr/i686-w64-mingw32/lib/libmsvcrt.a ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/i686-pc-windows-gnu/lib/

