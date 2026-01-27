use crate::traits::MutationStrategy;
use crate::types::Prompt;
use rand::{Rng, RngCore};

pub struct AppendMutator;

impl MutationStrategy<Prompt> for AppendMutator {
    fn mutate(&self, entity: &mut Prompt, _rng: &mut dyn RngCore) {
        entity.content.push_str("!");
    }
}

pub struct DeleteCharMutator;

impl MutationStrategy<Prompt> for DeleteCharMutator {
    fn mutate(&self, entity: &mut Prompt, rng: &mut dyn RngCore) {
        if entity.content.is_empty() {
            return;
        }

        let len = entity.content.chars().count();
        if len > 0 {
            let remove_index = rng.random_range(0..len);
            let new_content: String = entity
                .content
                .chars()
                .enumerate()
                .filter(|(i, _)| *i != remove_index)
                .map(|(_, c)| c)
                .collect();

            entity.content = new_content;
        }
    }
}
