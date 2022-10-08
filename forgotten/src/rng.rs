use lazy_static::lazy_static;
use parking_lot::Mutex;
use rand::{
    distributions::{
        uniform::{SampleRange, SampleUniform},
        Standard,
    },
    prelude::Distribution,
    seq::{SliceChooseIter, SliceRandom},
    Rng, RngCore, SeedableRng,
};
use rand_isaac::Isaac64Rng;

lazy_static! {
    pub static ref RNG: Mutex<Isaac64Rng> = Mutex::new(Isaac64Rng::from_entropy());
}

pub fn reseed_from_rng<R: RngCore>(rng: R) {
    *RNG.lock() = Isaac64Rng::from_rng(rng).expect("can't reseed rng");
}

pub fn reseed_u64(seed_u64: u64) {
    *RNG.lock() = Isaac64Rng::seed_from_u64(seed_u64);
}

pub fn choose<T>(slice: &[T]) -> Option<&T> {
    let mut rng = RNG.lock();
    slice.choose(&mut *rng)
}

pub fn choose_multiple<T>(slice: &[T], amount: usize) -> SliceChooseIter<[T], T> {
    let mut rng = RNG.lock();
    slice.choose_multiple(&mut *rng, amount)
}

pub fn shuffle<T>(slice: &mut [T]) {
    let mut rng = RNG.lock();
    slice.shuffle(&mut *rng)
}

pub fn range<T: SampleUniform, R: SampleRange<T>>(range: R) -> T {
    RNG.lock().gen_range(range)
}

pub fn gen<T>() -> T
where
    Standard: Distribution<T>,
{
    RNG.lock().gen::<T>()
}
