use optimize_engine::{Prompt, dummy_optimize};
use pyo3::prelude::*;
use pyo3::wrap_pymodule;

pub mod optimizers;
pub mod strategies;

#[pyfunction]
#[pyo3(text_signature = "(prompts)")]
fn run_dummy(prompts: Vec<String>) -> Vec<String> {
    let engine_prompts: Vec<Prompt> = prompts
        .into_iter()
        .map(|content| Prompt { content })
        .collect();

    dummy_optimize(engine_prompts)
}

#[pymodule]
fn _optimize_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(strategies::strategies))?;
    m.add_wrapped(wrap_pymodule!(optimizers::optimizers))?;

    let _sys_modules = m.py().import("sys")?.getattr("modules")?;

    m.add_function(wrap_pyfunction!(run_dummy, m)?)?;
    Ok(())
}
