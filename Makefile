CARGO_MANIFESTS=./Cargo.toml $(wildcard ./lang/**/Cargo.toml)
TEST_PROV=cargo test
TEST_PROV_ARGS=--manifest-path

test:
	cargo run --manifest-path ./test-suite/Cargo.toml

test-parser:
	cargo run --manifest-path ./test-suite/Cargo.toml -- parser

test-checker:
	cargo run --manifest-path ./test-suite/Cargo.toml -- typechecker

# test:
# 	echo Testing all crates
# 	$(foreach file, $(CARGO_MANIFESTS), $(TEST_PROV) $(TEST_PROV_ARGS) $(file);)

# test-parser:
# 	$(TEST_PROV) $(TEST_PROV_ARGS) ./lang/parser/Cargo.toml

# test-checker:
# 	$(TEST_PROV) $(TEST_PROV_ARGS) ./lang/checker/Cargo.toml
