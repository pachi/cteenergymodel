convert:
	cargo run -- ../hulc/src/ctehexml/BDCatalogo.bdc.utf8.gz
	cp ./hulcdb.rs ../bemodel/src/db/ctedb.rs

# Nota: usar Valgrind 3.17 al menos (ver snap)
massif:
	valgrind --tool=massif --massif-out-file=massiv.out.0 ../target/debug/convertdb ../hulc/src/ctehexml/BDCatalogo.bdc.utf8.gz
dhat:
	valgrind --tool=dhat --dhat-out-file=dhat.out.0 --mode=heap ../target/debug/convertdb ../hulc/src/ctehexml/BDCatalogo.bdc.utf8.gz
massif_view:
	ms_print massif.out.0
massif_visualizer:
	massif-visualizer massif.out.0
dhat_viewer:
	open https://nnethercote.github.io/dh_view/dh_view.html