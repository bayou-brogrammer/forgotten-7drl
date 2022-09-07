pub use rand::{Rng, SeedableRng};
pub use rand_isaac::Isaac64Rng;

pub enum InitialRngSeed {
    U64(u64),
    Random,
}

pub struct RngSeedSource {
    next_seed: u64,
    seed_rng: Isaac64Rng,
}

impl RngSeedSource {
    pub fn new(initial_rng_seed: InitialRngSeed) -> Self {
        let mut seed_rng = Isaac64Rng::from_entropy();
        let next_seed = match initial_rng_seed {
            InitialRngSeed::U64(seed) => seed,
            InitialRngSeed::Random => seed_rng.gen(),
        };
        Self { next_seed, seed_rng }
    }

    pub fn next_seed(&mut self) -> u64 {
        let seed = self.next_seed;
        self.next_seed = self.seed_rng.gen();

        #[cfg(feature = "print_stdout")]
        println!("RNG Seed: {}", seed);

        #[cfg(feature = "print_log")]
        log::info!("RNG Seed: {}", seed);

        seed
    }
}
