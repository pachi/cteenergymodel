#	rustup target add x86_64-pc-windows-gnu
#	rustup target add i686-pc-windows-gnu
win32:
	cargo build --release --target=i686-pc-windows-gnu
linux:
	cargo build --release
dist: linux win32
	mkdir -p dist
	cp target/i686-pc-windows-gnu/release/hulc2envolventecte.exe dist/
	cp target/release/hulc2envolventecte dist/
	strip dist/hulc2envolventecte.exe
	strip dist/hulc2envolventecte

