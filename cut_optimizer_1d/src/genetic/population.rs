use super::unit::Unit;

use rand::prelude::*;

use std::cmp::Ordering;
use std::mem;


struct LazyUnit<T: Unit> {
    unit: T,
    lazy_fitness: Option<f64>,
}

impl<T: Unit> LazyUnit<T> {
    fn from(unit: T) -> Self {
        LazyUnit {
            unit,
            lazy_fitness: None,
        }
    }

    fn fitness(&mut self) -> f64 {
        match self.lazy_fitness {
            Some(x) => x,
            None => {
                let fitness = self.unit.fitness();
                self.lazy_fitness = Some(fitness);
                fitness
            }
        }
    }
}

pub struct Population<T: Unit> {
    units: Vec<T>,

    seed: u64,
    breed_factor: f64,
    survival_factor: f64,
    max_size: usize,
}

impl<T: Unit> Population<T> {
    /// 新建实例
    pub fn new(init_pop: Vec<T>) -> Self {
        Population {
            units: init_pop,
            seed: 1,
            breed_factor: 0.5,
            survival_factor: 0.5,
            max_size: 100,
        }
    }

    //--------------------------------------------------------------------------

    /// rand seed
    pub fn set_rand_seed(&mut self, seed: u64) -> &mut Self {
        self.seed = seed;
        self
    }

    /// 设置最大数量
    pub fn set_size(&mut self, size: usize) -> &mut Self {
        self.units.truncate(size);
        self.max_size = size;
        self
    }

    /// factore
    pub fn set_breed_factor(&mut self, breed_factor: f64) -> &mut Self {
        assert!(breed_factor > 0.0 && breed_factor <= 1.0);
        self.breed_factor = breed_factor;
        self
    }

    /// 残余
    pub fn set_survival_factor(&mut self, survival_factor: f64) -> &mut Self {
        assert!((0.0..=1.0).contains(&survival_factor));
        self.survival_factor = survival_factor;
        self
    }

    //--------------------------------------------------------------------------

    /// 创建
    fn epoch(&self, units: &mut Vec<LazyUnit<T>>, mut rng: StdRng) -> StdRng {
        assert!(!units.is_empty());


        let breed_up_to = (self.breed_factor * (units.len() as f64)) as usize;
        let mut breeders: Vec<LazyUnit<T>> = Vec::new();

        while let Some(unit) = units.pop() {
            breeders.push(unit);
            if breeders.len() == breed_up_to {
                break;
            }
        }
        units.clear();


        let surviving_parents = (breeders.len() as f64 * self.survival_factor).ceil() as usize;

        for i in 0..self.max_size - surviving_parents {
            let rs = rng.gen_range(0..breeders.len());
            units.push(LazyUnit::from(
                breeders[i % breeders.len()]
                    .unit
                    .breed_with(&breeders[rs].unit, &mut rng),
            ));
        }

        //
        units.append(&mut breeders.drain(0..surviving_parents).collect());

        rng
    }

    ///
    pub fn epochs<F>(&mut self, n_epochs: u32, progress_callback: &F) -> &mut Self
    where
        F: Fn(f64),
    {
        let mut processed_stack = Vec::new();
        let mut active_stack = Vec::new();

        while let Some(unit) = self.units.pop() {
            active_stack.push(LazyUnit::from(unit));
        }

        let mut rng = SeedableRng::seed_from_u64(self.seed);

        for i in 0..=n_epochs {
            while let Some(mut unit) = active_stack.pop() {
                unit.fitness();
                processed_stack.push(unit);
            }


            mem::swap(&mut active_stack, &mut processed_stack);

            active_stack.sort_by(|a, b| {
                a.lazy_fitness
                    .unwrap_or(0.0)
                    .partial_cmp(&b.lazy_fitness.unwrap_or(0.0))
                    .unwrap_or(Ordering::Equal)
            });


            if active_stack.last().unwrap().lazy_fitness.unwrap_or(0.0) >= 1.0 {
                break;
            }

            if i != n_epochs {
                rng = self.epoch(&mut active_stack, rng);
            }

            progress_callback(i as f64 / n_epochs as f64);
        }


        while let Some(unit) = active_stack.pop() {
            self.units.push(unit.unit);
        }

        self
    }

    //--------------------------------------------------------------------------

    /// 交换结构
    pub fn finish(&mut self) -> Vec<T> {
        let mut empty_units: Vec<T> = Vec::new();
        mem::swap(&mut empty_units, &mut self.units);
        empty_units
    }
}
