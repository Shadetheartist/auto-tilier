use std::time::{Duration, Instant};
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use autotiler::{grid, tile}; // Replace with the actual crate and module names

fn benchmark_stripping_in_place(c: &mut Criterion) {
    let tile_set = tile::minimal_3x3_tile_set();

    let mut group = c.benchmark_group("stripping invalids in-place");

    for size in [8, 32, 128, 512, 2048].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            // custom benchmark to measure stripping in-place
            b.iter_custom(|iters| {
                let mut total_duration = Duration::ZERO;
                for _i in 0..iters {
                    let test_grid = grid::generate_test_grid(&tile_set, size, size);

                    // benchmarking only the actual stripping
                    let start = Instant::now();

                    let stripped_grid = grid::grid_strip_invalid(&test_grid);
                    black_box(stripped_grid);

                    total_duration += start.elapsed();

                }
                total_duration
            })
        });
    }

    group.finish()
}


criterion_group!(benches, benchmark_stripping_in_place);
criterion_main!(benches);