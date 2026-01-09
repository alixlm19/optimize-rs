use std::str::FromStr;

pub struct Prompt {
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub struct ParsePromptError;

impl FromStr for Prompt {
    type Err = ParsePromptError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Prompt {
            content: s.to_string(),
        })
    }
}

pub fn dummy_optimize(prompts: Vec<Prompt>) -> Vec<String> {
    prompts
        .into_iter()
        .map(|prompt| format!("{} - OPTIMIZED", prompt.content))
        .collect()
}
