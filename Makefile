run:
	$(info [INFO]: Ejecutando ejemplo)
	RUST_LOG=info cargo run --bin thor hulc_tests/tests/cubo/cubo.ctehexml -o cubo.json -r cubo_results.json
	RUST_LOG=info cargo run --bin thor hulc_tests/tests/e4h_medianeras/e4h_medianeras.ctehexml -o e4h_medianeras.json -r e4h_medianeras_results.json
	RUST_LOG=info cargo run --bin thor hulc_tests/tests/casoA/casoa.ctehexml -o caso_a.json -r caso_a_results.json
	RUST_LOG=info cargo run --bin thor hulc_tests/tests/ejemploviv_unif/ejemploviv_unif.ctehexml -o ejemploviv_unif.json -r ejemploviv_unif_results.json
	RUST_LOG=info cargo run --bin thor hulc_tests/tests/ejemplo_gt_aerotermia/ejemplo_gt_aerotermia.ctehexml -o ejemplo_gt_aerotermia.json -r ejemplo_gt_aerotermia_results.json
	$(info [INFO]: Actualizando ejemplos en bemodel/tests/data)
	mv e4h_medianeras.json e4h_medianeras_results.json bemodel/tests/data/
	mv caso_a.json caso_a_results.json bemodel/tests/data/
	mv cubo.json cubo_results.json bemodel/tests/data/
	mv ejemploviv_unif.json ejemploviv_unif_results.json bemodel/tests/data/
	mv ejemplo_gt_aerotermia.json ejemplo_gt_aerotermia_results.json bemodel/tests/data/
runskip:
	cargo run -- --skip-extra hulc_tests/tests/e4h_medianeras/ > e4h_medianeras.json.skip
	cargo run -- --skip-extra hulc_tests/tests/casoA/ > salida_a.json.skip
	cargo run -- --skip-extra hulc_tests/tests/e4h_medianeras/cubo.json > cubo.json.skip
compare: run runskip
	(meld bemodel/tests/e4h_medianeras/e4h_medianeras.json e4h_medianeras.json.skip&) && (meld -n bemodel/tests/e4h_medianeras/caso_a.json salida_a.json.skip&) && (meld -n bemodel/tests/e4h_medianeras/cubo.json cubo.json.skip&)
climatedata:
	# cargo run --release --bin metconvert ../climascte
	cargo run --release --bin metconvert -- --pretty ../climascte
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
# Instala linux-perf: sudo apt install -y linux-tools-generic
# echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
# Configuración permanente en /etc/sysctl.conf con kernel.perf_event_paranoid = -1
# Instala cargo-flamegraph: cargo install flamegraph
# https://github.com/flamegraph-rs/flamegraph
# después de make profile, visualizar flamegraph.svg en el navegador...
profile:
	cargo flamegraph --bench benchmark_bemodel -- --bench
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

thor:
	cargo run --bin thor ${input} -v -o model_thor.json -r results_thor.json
