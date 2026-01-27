from typing import List

from ..strategies import StrategyType

class PromptOptimizer:
    """
    A genetic optimizer for text prompts.
    """

    mutation_rate: float
    """The probability (0.0 to 1.0) that a mutation will occur."""

    def __init__(
        self, texts: List[str], rate: float, strategy: StrategyType
    ) -> None: ...
    def step(self) -> None: ...
    def get_prompts(self) -> List[str]: ...
