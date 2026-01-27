use optimize_engine::Prompt;
use optimize_engine::genetic::GeneticOptimizer;
use optimize_engine::strategies::{AppendMutator, DeleteCharMutator};
use optimize_engine::traits::MutationStrategy;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::strategies::StrategyType;

#[pyclass]
pub struct PromptOptimizer {
    inner: GeneticOptimizer<Prompt>,
}

#[pymethods]
impl PromptOptimizer {
    #[new]
    #[pyo3(text_signature = "(texts, mutation_rate, strategy)")]
    fn new(texts: Vec<String>, mutation_rate: f64, strategy: StrategyType) -> Self {
        let prompts: Vec<Prompt> = texts.into_iter().map(|s| Prompt { content: s }).collect();

        let mutator: Box<dyn MutationStrategy<Prompt>> = match strategy {
            StrategyType::Append => Box::new(AppendMutator),
            StrategyType::Delete => Box::new(DeleteCharMutator),
        };

        PromptOptimizer {
            inner: GeneticOptimizer::new(prompts, mutation_rate, mutator),
        }
    }

    /// Run one step of evolution
    #[pyo3(text_signature = "($self)")]
    fn step(&mut self) -> PyResult<()> {
        self.inner
            .step(|_| 0.0)
            .map_err(|e| PyValueError::new_err(e))
    }

    /// Get the current population as a list of strings
    #[pyo3(text_signature = "($self)")]
    fn get_prompts(&self) -> Vec<String> {
        self.inner
            .population
            .iter()
            .map(|p| p.content.clone())
            .collect()
    }

    /// Allow Python to read/write the mutation rate dynamically
    #[getter]
    fn get_mutation_rate(&self) -> f64 {
        self.inner.mutation_rate
    }

    #[setter]
    fn set_mutation_rate(&mut self, mutation_rate: f64) -> PyResult<()> {
        if !(0.0..=1.0).contains(&mutation_rate) {
            return Err(PyValueError::new_err(
                "Mutation rate must be between 0.0 and 1.0",
            ));
        }
        self.inner.mutation_rate = mutation_rate;
        Ok(())
    }
}

// 2. Define the submodule initialization function
#[pymodule]
pub fn genetic(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PromptOptimizer>()?;
    Ok(())
}
