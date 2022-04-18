check:
	cargo run -- ../hulc_tests/tests/e4h_medianeras/ > test.json
	cargo run -- --use-extra ../hulc_tests/tests/e4h_medianeras/ > test_extra.json
	kdiff3 test.json test_extra.json