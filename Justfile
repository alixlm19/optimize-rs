# List available commands
default:
    @just --list

# Build and install (Ensure __init__.py exists first!)
dev:
    # 1. Create the package init if it doesn't exist (prevents the maturin warning)
    #    (Using '>>' appends instead of overwriting, or usage of 'test' to check existence)
    @if [ ! -f pysrc/optimize_rs/__init__.py ]; then \
        mkdir -p pysrc/optimize_rs; \
        echo 'from ._optimize_rs import *' > pysrc/optimize_rs/__init__.py; \
    fi
    
    # 2. Build
    uv run maturin develop
    
    # 3. Update Stubs
    @just stubs

# Generate type stubs
stubs:
    # 1. Generate stubs for the binary module
    uv run stubgen -m optimize_rs._optimize_rs -o .
    
    # 2. Move and rename to __init__.pyi (The Type Definition)
    #    Note: We do NOT touch __init__.py (The Runtime Code) here.
    mv optimize_rs/_optimize_rs.pyi pysrc/optimize_rs/__init__.pyi
    
    # 3. Cleanup
    rm -rf optimize_rs

# Run the test script
run-test:
    # 👈 Fixed: Point to the actual file
    uv run python pysrc/main.py
