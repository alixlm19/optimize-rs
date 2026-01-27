class StrategyType:
    """
    Available mutation strategies.
    """

    Append: "StrategyType"
    Delete: "StrategyType"

    def __eq__(self, other: object) -> bool: ...
