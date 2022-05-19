DEFAULT_FEATURES = --features no-out
TEST_SUITE_MANIFEST = --manifest-path ./test-suite/Cargo.toml

default:
all:

test:
	cargo run $(TEST_SUITE_MANIFEST)

test-no-out:
	cargo run $(DEFAULT_FEATURES) $(TEST_SUITE_MANIFEST)

parse-test:
	cargo run $(DEFAULT_FEATURES) $(TEST_SUITE_MANIFEST) -- parser

typ-test:
	cargo run $(DEFAULT_FEATURES) $(TEST_SUITE_MANIFEST) -- typechecker

typ-debug:
	cargo run $(TEST_SUITE_MANIFEST) $(DEFAULT_FEATURES) --features debug -- typechecker

# test:
# 	echo Testing all crates
# 	$(foreach file, $(CARGO_MANIFESTS), $(TEST_PROV) $(TEST_PROV_ARGS) $(file);)

# test-parser:
# 	$(TEST_PROV) $(TEST_PROV_ARGS) ./lang/parser/Cargo.toml

# test-checker:
# 	$(TEST_PROV) $(TEST_PROV_ARGS) ./lang/checker/Cargo.toml
