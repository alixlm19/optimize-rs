# optimize-rs

High-performance optimization library written in Rust with Python bindings.

`optimize-rs` provides a generic engine for evolutionary algorithms, specifically designed to be efficient and extensible. While the core logic is implemented in Rust for performance, it exposes a user-friendly Python API for easy integration into existing workflows (e.g., prompt engineering, hyperparameter tuning).

## Features

- 🧬 **Genetic Algorithm Engine**: Robust implementation of evolutionary strategies including selection, crossover, and mutation.
- 🚀 **High Performance**: Core logic written in Rust to minimize overhead during iterative optimization steps.
- 🐍 **Python Bindings**: Seamless integration with Python using `PyO3` and `maturin`.
- 🧩 **Modular Strategies**: Pluggable mutation and crossover strategies.

## Project Structure

This project is organized as a Rust workspace with mixed Python support:

- `crates/engine`: The core Rust library containing the optimization logic and generic traits.
- `crates/binding`: The PyO3 bridge that exposes Rust types to Python.
- `crates/cli`: A command-line interface for running optimizations directly (in development).
- `pysrc/`: The Python source package (`optimize_rs`) and type stubs.

## Prerequisites

- **Rust**: Latest stable toolchain.
- **Python**: 3.14+ (as specified in configuration, though likely compatible with 3.10+).
- **uv**: Fast Python package installer and resolver.
- **Just**: A handy command runner (optional, but recommended).

## Installation & Development

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/optimize-rs.git
   cd optimize-rs
   ```

2. **Setup Python Environment:**
   We use `uv` for dependency management.
   ```bash
   uv sync
   ```

3. **Build Python Bindings:**
   Use `just` (or `maturin` directly) to build the extension and install it in editable mode.
   ```bash
   just dev
   # OR
   uv run maturin develop
   ```

4. **Run Tests:**
   Verify the installation by running the example script.
   ```bash
   just run-test
   # OR
   uv run python pysrc/main.py
   ```

## Usage (Python)

Here is a simple example of how to use the Genetic Optimizer for prompt optimization:

```python
from optimize_rs.optimizers import genetic
from optimize_rs.strategies import StrategyType

def main():
    # 1. Define your initial population
    prompts = ["Hello World"]

    # 2. Initialize the Optimizer
    # Arguments: initial_population, mutation_rate, mutation_strategy
    opt = genetic.PromptOptimizer(prompts, 0.5, StrategyType.Append)

    print(f"Initial: {opt.get_prompts()}")

    # 3. Evolve the population
    opt.step()

    print(f"Evolved: {opt.get_prompts()}")

if __name__ == "__main__":
    main()
```

## Usage (Rust)

To use the engine directly in a Rust project, add `optimize-engine` to your dependencies.

```rust
use optimize_engine::genetic::GeneticOptimizer;
// Implement the Evolvable trait for your custom type...
```

## License

[MIT](LICENSE)