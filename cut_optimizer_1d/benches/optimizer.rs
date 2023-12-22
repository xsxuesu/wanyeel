use std::time::Duration;

use criterion::*;
use cut_optimizer_1d::*;
use rand::prelude::*;

fn build_optimizer() -> Optimizer {
    let mut rng: StdRng = SeedableRng::seed_from_u64(1);

    let mut optimizer = Optimizer::new();
    optimizer.add_stock_piece(StockPiece {
        length: 96,
        weight: 0,
        quantity: None,
    });
    optimizer.add_stock_piece(StockPiece {
        length: 96,
        weight: 0,
        quantity: None,
    });
    optimizer.add_stock_piece(StockPiece {
        length: 120,
        weight: 0,
        quantity: None,
    });
    optimizer.add_stock_piece(StockPiece {
        length: 120,
        weight: 0,
        quantity: None,
    });

    let num_cut_pieces = 20;

    for i in 0..num_cut_pieces {
        optimizer.add_cut_piece(CutPiece {
            quantity: 1,
            external_id: Some(i),
            length: rng.gen_range(1..=120),
        });
    }

    optimizer
}

pub fn benchmark_optimize(c: &mut Criterion) {
    c.bench_function("optimize random cut pieces", |b| {
        b.iter(|| {
            let _ = build_optimizer()
                .set_cut_width(1)
                .set_random_seed(1)
                .optimize(|_| {});
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(20));
    targets = benchmark_optimize
}

criterion_main!(benches);
