pub mod genetic;
pub mod strategies;
pub mod traits;
pub mod types;

pub use types::Prompt;

pub fn dummy_optimize(prompts: Vec<Prompt>) -> Vec<String> {
    prompts
        .into_iter()
        .map(|prompt| format!("{} - OPTIMIZED", prompt.content))
        .collect()
}
