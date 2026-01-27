from optimize_rs.optimizers import genetic
from optimize_rs.strategies import StrategyType


def main():
    # 1. Setup
    prompts = ["Hello World"]

    # 2. Initialize Optimizer (calls Rust `new`)
    opt = genetic.PromptOptimizer(prompts, 0.5, StrategyType.Append)

    # 3. Run a step (calls Rust `step`)
    print(f"Before: {opt.get_prompts()}")
    opt.step()

    # 4. Check result (calls Rust `get_prompts`)
    print(f"After:  {opt.get_prompts()}")
    print("✅ Rust integration successful!")


if __name__ == "__main__":
    main()
