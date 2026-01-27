use crate::traits::{Evolvable, MutationStrategy};

use rand::prelude::*;
use std::marker::PhantomData;

pub struct GeneticOptimizer<T> {
    pub population: Vec<T>,
    pub mutation_rate: f64,
    pub mutator: Box<dyn MutationStrategy<T>>,

    _marker: PhantomData<T>,
}

impl<T: Evolvable> GeneticOptimizer<T> {
    pub fn new(
        initial_pop: Vec<T>,
        mutation_rate: f64,
        mutator: Box<dyn MutationStrategy<T>>,
    ) -> Self {
        Self {
            population: initial_pop,
            mutation_rate,
            mutator,
            _marker: PhantomData,
        }
    }

    pub fn step<F>(&mut self, _fitness_fn: F) -> Result<(), String>
    where
        F: Fn(&T) -> f64,
    {
        let mut rng = rand::rng();

        if self.population.is_empty() {
            return Err("Cannot run step: population is emtpy.".to_string());
        }

        let new_pop_size = self.population.len();
        let mut next_gen = Vec::with_capacity(new_pop_size);

        for _ in 0..new_pop_size {
            let parent_a = self
                .population
                .choose(&mut rng)
                .ok_or("Failed to select parent A")?;

            let parent_b = self
                .population
                .choose(&mut rng)
                .ok_or("Failed to select parent B")?;

            let mut child = parent_a.crossover(parent_b, &mut rng);
            if rng.random_bool(self.mutation_rate) {
                self.mutator.mutate(&mut child, &mut rng);
            }

            next_gen.push(child);
        }

        self.population = next_gen;

        Ok(())
    }
}
