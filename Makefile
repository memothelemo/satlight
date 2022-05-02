CARGO_MANIFESTS=./Cargo.toml $(wildcard ./lang/**/Cargo.toml)
TEST_PROV=nextest run
TEST_PROV_ARGS=--manifest-path

# nextest doesn't test all of the crates natively
test:
	echo Testing all crates
	$(foreach file, $(CARGO_MANIFESTS), cargo $(TEST_PROV) $(TEST_PROV_ARGS) $(file);)
