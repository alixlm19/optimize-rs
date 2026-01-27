use crate::traits::Evolvable;
use rand::RngCore;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
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

impl Evolvable for Prompt {
    fn crossover(&self, _partner: &Self, _rng: &mut dyn RngCore) -> Self {
        let cut_a = self.content.len() / 2;
        let cut_b = _partner.content.len() / 2;

        let new_content = format!("{}{}", &self.content[0..cut_a], &self.content[cut_b..]);

        Prompt {
            content: new_content,
        }
    }
}
