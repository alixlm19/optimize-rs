use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};

pub mod genetic;

#[pymodule]
pub fn optimizers(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();

    let genetic_mod = PyModule::new(py, "genetic")?;

    // 2. Initialize it
    genetic::genetic(&genetic_mod)?;

    // 3. Attach to parent
    m.add_submodule(&genetic_mod)?;

    // 4. Register in sys.modules
    let parent_name = m.name()?;
    let full_name = format!("{}.genetic", parent_name);

    let sys = py.import("sys")?;
    let modules = sys.getattr("modules")?.cast_into::<PyDict>()?;
    modules.set_item(full_name, &genetic_mod)?;

    Ok(())
}
