run:
	$(info [INFO]: Ejecutando ejemplo)
	RUST_LOG=info cargo run hulc_tests/tests/cubo/ > cubo.json
	RUST_LOG=info cargo run hulc_tests/tests/data/ > e4h_medianeras.json
	RUST_LOG=info cargo run hulc_tests/tests/casoA/ > caso_a.json
	$(info [INFO]: Actualizando ejemplos en bemodel/tests/data)
	mv e4h_medianeras.json bemodel/tests/data/e4h_medianeras.json
	mv caso_a.json bemodel/tests/data/caso_a.json
	mv cubo.json bemodel/tests/data/cubo.json
runskip:
	cargo run -- --skip-extra hulc_tests/tests/data/ > e4h_medianeras.json.skip
	cargo run -- --skip-extra hulc_tests/tests/casoA/ > salida_a.json.skip
	cargo run -- --skip-extra hulc_tests/tests/data/cubo.json > cubo.json.skip
compare: run runskip
	(meld bemodel/tests/data/e4h_medianeras.json e4h_medianeras.json.skip&) && (meld -n bemodel/tests/data/caso_a.json salida_a.json.skip&) && (meld -n bemodel/tests/data/cubo.json cubo.json.skip&)
climatedata:
	cargo run --bin metconvert ../climascte
linux:
	$(info [INFO]: Versión de producción para linux)
	cargo build --release
win32:
	$(info [INFO]: Versión de producción para i686-pc-windows-gnu)
	cargo build --release --target=i686-pc-windows-gnu
release: linux win32
	$(info [INFO]: Compilando versión de producción)
	mkdir -p dist
	cp target/i686-pc-windows-gnu/release/hulc2model.exe dist/
	cp target/release/hulc2model dist/
	strip dist/hulc2model.exe
	strip dist/hulc2model
bloat:
	$(info [INFO]: Calculando consumo de espacio en archivo ejecutable)
	cargo bloat --release -n 10
	cargo bloat --release --crates -n 10
	# cargo bloat --release --crates --split-std -n 10
bench:
	cargo bench
perf:
	# Ver https://profiler.firefox.com/docs/#/./guide-perf-profiling
	$(info [INFO]: Obteniendo datos de perfilado en el archivo perf.data)
	# -g -> call graph recording, -F number -> frecuencia de registro
	$(info [INFO]: Puede parar la toma de datos cuando quiera con Ctrl+C)
	perf record -g -F 999 target/release/hulc2model hulc_tests/tests/data > /dev/null
	$(info [INFO]: Convirtiendo datos de perfilado a Firefox Profiler y guardando en /tmp/test.perf)
	perf script -F +pid > /tmp/test.perf
	$(info [INFO]: Puede abrir los datos en http://profiler.firefox.com)
	# Visualización alterantiva
	# perf report -f --sort comm,dso
count:
	$(info [INFO]: Calculando tamaño del código)
	cargo count -a src/
outdated:
	$(info [INFO]: Dependencias obsoletas)
	cargo outdated -R
fixcross:
	$(info [INFO]: Reparando compilación cruzada desde linux a i686-pc-windows-gnu)
	sudo aptitude install -y mingw-w64 mingw-w64-tools
	cp /usr/i686-w64-mingw32/lib/crt2.o ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/i686-pc-windows-gnu/lib/
	cp /usr/i686-w64-mingw32/lib/dllcrt2.o ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/i686-pc-windows-gnu/lib/
	cp /usr/i686-w64-mingw32/lib/libmsvcrt.a ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/i686-pc-windows-gnu/lib/

