default: run_dev

clean:
	@echo "Cleaning using cargo..."
	@cargo clean

check:
	@echo "Checking..."
	@cargo check --features dotenv

build_dev:
	@echo "Building debug..."
	@cargo build --features dotenv

build_release:
	@echo "Building release..."
	@cargo build --release

run_dev:
	@echo "Running debug..."
	@cargo run --features dotenv

run_release:
	@echo "Running release..."
	@cargo run --release
