// crates/binding/src/strategies.rs
use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum StrategyType {
    Append,
    Delete,
}

// 1. Define the submodule initialization function
#[pymodule]
pub fn strategies(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StrategyType>()?;
    Ok(())
}
