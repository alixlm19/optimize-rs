use rand::RngCore;

/// Genetic Algorithm Data Contract
pub trait Evolvable: Clone + Send + Sync {
    /// Combine with another parent to produce a child
    fn crossover(&self, _partner: &Self, _rng: &mut dyn RngCore) -> Self;
}

/// Mutation Strategy Contract
pub trait MutationStrategy<T>: Send + Sync {
    fn mutate(&self, entity: &mut T, rng: &mut dyn RngCore);
}
