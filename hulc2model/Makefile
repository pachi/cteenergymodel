check:
	cargo run -- ../hulc_tests/tests/data/ > test.json
	cargo run -- --use-extra ../hulc_tests/tests/data/ > test_extra.json
	kdiff3 test.json test_extra.json