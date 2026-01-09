use pyo3::prelude::*;

use optimize_engine::{Prompt, dummy_optimize};

#[pyfunction]
fn run_dummy(prompts: Vec<String>) -> Vec<String> {
    let engine_prompts: Vec<Prompt> = prompts
        .into_iter()
        .map(|content| Prompt { content })
        .collect();

    dummy_optimize(engine_prompts)
}

#[pymodule]
fn _optimize_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_dummy, m)?)?;
    Ok(())
}
