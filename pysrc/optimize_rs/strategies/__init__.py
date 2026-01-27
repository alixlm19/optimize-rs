# pysrc/optimize_rs/strategies/__init__.py

# 1. Import the binary module from the parent package
from .. import _optimize_rs

# 2. Access the 'strategies' submodule and 'StrategyType' class via attributes
#    (We cannot do 'from .._optimize_rs.strategies import StrategyType')
StrategyType = _optimize_rs.strategies.StrategyType

# 3. Export it
__all__ = ["StrategyType"]
