use rand::prelude::*;

pub trait Unit: Send {
    fn fitness(&self) -> f64;
    fn breed_with<R>(&self, other: &Self, rng: &mut R) -> Self
    where
        R: Rng + ?Sized;
}
