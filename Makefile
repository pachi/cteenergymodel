linux:
	cargo build --release
win32:
	cargo build --release --target=i686-pc-windows-gnu
dist: linux win32
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
	perf record -g target/release/hulc2envolventecte src/data
	perf report -f --sort comm,dso
