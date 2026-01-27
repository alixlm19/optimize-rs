# List available commands
default:
    @just --list

dev:
    uv run maturin develop

# Run the test script
run-test:
    uv run python pysrc/main.py
